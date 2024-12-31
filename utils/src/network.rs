use crate::Message;
use log::info;
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;

pub struct Network {
    pub id: i32,                            // self id
    pub listen_addr: String,                // self listen address
    pub remote_addrs: HashMap<i32, String>, // set of remote peers to connect

    pub outgoing_connections: HashMap<i32, Arc<Mutex<TcpStream>>>, // for each remote peer the outgoing tcp channel

    pub incoming_channel_sender: Arc<mpsc::Sender<Message>>, // listening sockets will put messages to incoming chan using this object
    pub incoming_channel_receiver: Arc<mpsc::Receiver<Message>>, // network main thread will poll messages from incoming channel using this object

    pub external_outgoing_channel_sender: Arc<mpsc::Sender<Message>>, // all messages from the incoming channel will be sent to external channel using this object, and will be consumed by a different thread
}
impl Network {
    pub fn new(
        id: i32,
        listen: String,
        remotes: HashMap<i32, String>,
        outgoing_channel_sender: Arc<mpsc::Sender<Message>>,
    ) -> Network {
        let (tx, rx) = mpsc::channel(10_000);
        Self {
            id: id,
            listen_addr: listen,
            remote_addrs: remotes,
            outgoing_connections: HashMap::new(),
            incoming_channel_sender: Arc::new(tx),
            incoming_channel_receiver: Arc::new(rx),
            external_outgoing_channel_sender: outgoing_channel_sender.clone(),
        }
    }

    // add a new outgoing tcp connection
    pub fn add_outgoing_connection(&mut self, id: i32, stream: TcpStream) {
        if self.outgoing_connections.contains_key(&id) {
            self.outgoing_connections.remove(&id);
        }
        self.outgoing_connections
            .insert(id, Arc::new(Mutex::new(stream)));
    }

    pub fn remove_outgoing_connection(&mut self, id: i32) {
        self.outgoing_connections.remove(&id);
    }

    pub fn handle_incoming_messages(&mut self) {
        thread::spawn({
            loop {
                self.external_outgoing_channel_sender
                    .blocking_send(self.incoming_channel_receiver.recv())
                    .unwrap()
            }
        });
    }

    pub fn start_listening(&mut self) {
        let listen_addr = self.listen_addr.clone();
        let sender = Arc::clone(&self.incoming_channel_sender); // Clone the Arc

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                match TcpListener::bind(&listen_addr).await {
                    Ok(listener) => {
                        println!("server listening on {}", listen_addr);

                        loop {
                            match listener.accept().await {
                                Ok((mut socket, addr)) => {
                                    println!("Accepted connection from {:?}", addr);
                                    let local_sender = Arc::clone(&sender);
                                    self.listen_connection(local_sender, socket, addr);
                                }
                                Err(e) => {
                                    panic!("Failed to accept connection: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => panic!("Failed to bind to {}: {}", listen_addr, e),
                }
            });
        });
    }

    pub fn listen_connection(
        &mut self,
        sender: Arc<Sender<Message>>,
        mut socket: TcpStream,
        addr: SocketAddr,
    ) {
        thread::spawn(async move || {
            println!("new connection from {:?}", addr);
            let mut buffer = vec![0; 1024 * 1024 * 10]; // max 10 MB messages
            loop {
                if let Ok(size) = socket.read(&mut buffer).await {
                    if size > 0 {
                        match serde_json::from_slice::<Message>(&buffer[..size]) {
                            Ok(message) => {
                                sender.send(message).await.unwrap();
                                info!("sent message to network internal chan {?}", message)
                            }
                            Err(e) => {
                                eprintln!("failed to deserialize message from {}", addr);
                                if let Ok(..) = socket.shutdown().await.unwrap() {
                                    eprintln!("closed incoming connection from {}", addr);
                                }
                                break;
                            }
                        }
                    }
                }
            }
        });
    }

    pub async fn initiate_all_connections(&mut self) {
        for (id, addr) in &self.remote_addrs {
            loop {
                match TcpStream::connect(addr).await {
                    Ok(stream) => {
                        println!("Successfully connected to remote peer {} at {}", id, addr);
                        self.add_outgoing_connection(*id, stream);
                        break;
                    }
                    Err(e) => {
                        eprintln!("Failed to connect to peer {} at {}: {}", id, addr, e);
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        }
    }

    pub async fn initiate_connection(&mut self, id: i32) {
        if !self.remote_addrs.contains_key(&id) {
            panic!("No such address: {}", id);
        }

        let addr = self.remote_addrs.get(&id).unwrap();

        match TcpStream::connect(addr).await {
            Ok(stream) => {
                println!("Successfully connected to remote peer {} at {}", id, addr);
                self.add_outgoing_connection(id, stream);
            }
            Err(e) => {
                eprintln!("Failed to connect to peer {} at {}: {}", id, addr, e);
            }
        }
    }

    pub async fn send_message(&mut self, message: Message, id: i32) {
        if !self.outgoing_connections.contains_key(&id) {
            panic!("No such outgoing connection: {}", id);
        }
        let stream = self.outgoing_connections.get(&id).unwrap();
        let serialized_message = serde_json::to_vec(&message)?;
        match stream.write_all(&serialized_message).await {
            Ok(_) => {
                println!("Sent message to network outgoing connection {}", id);
            }
            Err(e) => {
                eprintln!(
                    "Failed to send message to network outgoing connection {}: {}",
                    id, e
                );
                self.initiate_connection(id).await;
            }
        }
    }
}

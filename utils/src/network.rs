use crate::Message;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct Network {
    pub id: i32,                            // self id
    pub listen_addr: String,                // self listen address
    pub remote_addrs: HashMap<i32, String>, // set of remote peers to connect

    pub outgoing_connections: HashMap<i32, TcpStream>, // for each remote peer the outgoing tcp channel

    pub incoming_channel_sender: Sender<Message>, // listening sockets will put messages to incoming chan using this object
    pub incoming_channel_receiver: Receiver<Message>, // network main thread will poll messages from incoming channel using this object

    pub external_outgoing_channel_sender: Sender<Message>, // all messages from the incoming channel will be sent to external channel using this object, and will be consumed by a different thread
}
impl Network {
    pub fn new(
        id: i32,
        listen: String,
        remotes: HashMap<i32, String>,
        outgoing_channel_sender: Sender<Message>,
    ) -> Network {
        let (tx, rx) = mpsc::channel(10_000);
        Self {
            id: id,
            listen_addr: listen,
            remote_addrs: remotes,
            outgoing_connections: HashMap::new(),
            incoming_channel_sender: tx,
            incoming_channel_receiver: rx,
            external_outgoing_channel_sender: outgoing_channel_sender,
        }
    }

    // add a new outgoing tcp connection
    pub fn add_outgoing_connection(&mut self, id: i32, stream: TcpStream) {
        if self.outgoing_connections.contains_key(&id) {
            self.outgoing_connections.remove(&id);
        }
        self.outgoing_connections.insert(id, stream);
    }

    pub fn remove_outgoing_connection(&mut self, id: i32) {
        self.outgoing_connections.remove(&id);
    }

    // pub async fn handle_incoming_messages(self: Arc<Self>) {
    //     let mut receiver = &self.incoming_channel_receiver; // Move or clone the receiver
    //     let sender = self.external_outgoing_channel_sender.clone(); // Clone the sender
    //
    //     tokio::spawn(async move {
    //         while let Ok(message) = receiver.recv() {
    //             // Forward the message to the external outgoing channel
    //             if let Err(e) = sender.send(message).await {
    //                 eprintln!("Error sending message: {:?}", e);
    //             }
    //         }
    //
    //         // Log if the channel is closed
    //         eprintln!("Incoming channel is closed. Exiting message handling loop.");
    //     });
    // }
    //
    // pub fn start_listening(&mut self) {
    //     let listen_addr = self.listen_addr.clone();
    //
    //     thread::spawn(move || {
    //         let rt = tokio::runtime::Runtime::new().unwrap();
    //         rt.block_on(async move {
    //             match TcpListener::bind(&listen_addr).await {
    //                 Ok(listener) => {
    //                     println!("server listening on {}", listen_addr);
    //
    //                     loop {
    //                         match listener.accept().await {
    //                             Ok((mut socket, addr)) => {
    //                                 println!("Accepted connection from {:?}", addr);
    //                                 self.listen_connection(socket, addr);
    //                             }
    //                             Err(e) => {
    //                                 panic!("Failed to accept connection: {}", e);
    //                             }
    //                         }
    //                     }
    //                 }
    //                 Err(e) => panic!("Failed to bind to {}: {}", listen_addr, e),
    //             }
    //         });
    //     });
    // }
    //
    // pub fn listen_connection(&mut self, mut socket: TcpStream, addr: SocketAddr) {
    //     let sender = Arc::clone(&self.incoming_channel_sender);
    //     thread::spawn({
    //         let sender = sender.clone();
    //         async move {
    //             println!("new connection from {}", addr);
    //             let mut buffer = vec![0; 1024 * 1024 * 10]; // max 10 MB messages
    //             loop {
    //                 match socket.read(&mut buffer).await {
    //                     Ok(size) if size > 0 => {
    //                         match serde_json::from_slice::<Message>(&buffer[..size]) {
    //                             Ok(message) => {
    //                                 if let Err(e) = sender.send(message).await {
    //                                     eprintln!("failed to send message to channel: {}", e);
    //                                 } else {
    //                                     info!("sent message to network internal chan");
    //                                 }
    //                             }
    //                             Err(e) => {
    //                                 eprintln!(
    //                                     "failed to deserialize message from {}: {:?}",
    //                                     addr, e
    //                                 );
    //                                 if socket.shutdown().await.is_ok() {
    //                                     eprintln!("closed incoming connection from {}", addr);
    //                                 }
    //                                 break;
    //                             }
    //                         }
    //                     }
    //                     Err(e) => {
    //                         eprintln!("failed to read from socket {}: {:?}", addr, e);
    //                         if socket.shutdown().await.is_ok() {
    //                             eprintln!("closed incoming connection from {}", addr);
    //                         }
    //                         break;
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //     });
    // }
    //
    // pub async fn initiate_all_connections(&mut self) {
    //     let remote_addrs = self.remote_addrs.clone();
    //     for (id, addr) in &remote_addrs {
    //         loop {
    //             match TcpStream::connect(addr).await {
    //                 Ok(stream) => {
    //                     println!("Successfully connected to remote peer {} at {}", id, addr);
    //                     self.add_outgoing_connection(*id, stream);
    //                     break;
    //                 }
    //                 Err(e) => {
    //                     eprintln!("Failed to connect to peer {} at {}: {}", id, addr, e);
    //                     sleep(Duration::from_secs(1)).await;
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    // pub async fn initiate_connection(&mut self, id: i32) {
    //     if !self.remote_addrs.contains_key(&id) {
    //         panic!("No such address: {}", id);
    //     }
    //
    //     let addr = self.remote_addrs.get(&id).unwrap();
    //
    //     match TcpStream::connect(addr).await {
    //         Ok(stream) => {
    //             println!("Successfully connected to remote peer {} at {}", id, addr);
    //             self.add_outgoing_connection(id, stream);
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to connect to peer {} at {}: {}", id, addr, e);
    //         }
    //     }
    // }
    //
    // pub async fn send_message(&mut self, message: Message, id: i32) {
    //     if !self.outgoing_connections.contains_key(&id) {
    //         panic!("No such outgoing connection: {}", id);
    //     }
    //
    //     // Get the stream and lock it asynchronously
    //     let stream = match self.outgoing_connections.get(&id) {
    //         Some(s) => s.clone(),
    //         None => {
    //             panic!("should not happen")
    //         }
    //     };
    //     let mut stream = stream.lock().await;
    //
    //     // Serialize the message and handle serialization errors
    //     let serialized_message = match serde_json::to_vec(&message) {
    //         Ok(data) => data,
    //         Err(e) => {
    //             eprintln!("Failed to serialize message for connection {}: {}", id, e);
    //             panic!("Failed to serialize message for connection {}: {}", id, e);
    //         }
    //     };
    //
    //     // Write the serialized message to the stream
    //     match stream.write_all(&serialized_message).await {
    //         Ok(_) => {
    //             println!("Sent message to network outgoing connection {}", id);
    //         }
    //         Err(e) => {
    //             eprintln!(
    //                 "Failed to send message to network outgoing connection {}: {}",
    //                 id, e
    //             );
    //             self.initiate_connection(id); // Attempt to reconnect on failure
    //         }
    //     }
    // }
}

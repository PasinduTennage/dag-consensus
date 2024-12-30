use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use utils::{ClientRequest, Message, ReplicaMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("server listening");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];
            if let Ok(size) = socket.read(&mut buffer).await {
                if size > 0 {
                    match serde_json::from_slice::<Message>(&buffer[..size]) {
                        Ok(message) => {
                            match message {
                                Message::ClientRequest(client_request) => {
                                    println!("received ClientRequest: {:?}", client_request);
                                }
                                Message::ReplicaMessage(replica_message) => {
                                    println!("received ReplicaMessage: {:?}", replica_message);
                                }
                            }
                            if let Err(e) = socket.write_all(&buffer[..size]).await {
                                eprintln!("failed to send response {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("failed to deserialize {}", e);
                        }
                    }
                }
            }
        });
    }
}

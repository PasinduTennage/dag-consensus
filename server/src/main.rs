use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use utils::ClientRequest;

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
                    match serde_json::from_slice::<ClientRequest>(&buffer[..size]) {
                        Ok(message) => {
                            println!("received: {:?}", message);
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

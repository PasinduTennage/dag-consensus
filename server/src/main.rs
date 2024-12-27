use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("server listening");

    loop{
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move{
            let mut buffer = vec![0; 1024];
            if let Ok(size) = socket.read(&mut buffer).await{
                if size > 0 {
                    let message: Value = serde_json::from_slice(&buffer[..size]).unwrap();
                    println!("Received: {:?}", message);

                    if let Err(e) = socket.write_all(&buffer[..size]).await{
                        eprintln!("failed to send repsonse: {}", e)
                    }
                }

            }
        });

    }

}

use serde_json::error::Category::Data;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::json;
use utils::ClientRequest;

#[tokio::main]
async fn main() -> Result <(), Box<dyn std::error::Error>>{

    for i in 0..100{
        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

        println!("connected to server");
        let message = ClientRequest{
            id: format!("message-{}", i),
            payload: vec![1,2,3,4],
            sender: i,
        };

        let serialized_message = serde_json::to_vec(&message)?;

        stream.write_all(&serialized_message).await?;

        println!("sent {:?}", serialized_message);

        let mut buffer = vec![0; 1024];

        let size = stream.read(&mut buffer).await?;

        if size > 0 {
            let response = String::from_utf8_lossy(&buffer[..size]);
            println!("Received: {:?}", response);
        }

    }



    Ok(())
}

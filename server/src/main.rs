use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("server listening");

    loop {
        tokio::spawn(async move {});
    }
}

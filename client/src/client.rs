use tokio::net::TcpStream;
use tokio::io::{ AsyncReadExt,AsyncWriteExt};
use utils::ClientRequest;
use serde_json;

pub struct Client{
    pub id: i32,
}

impl Client{

    pub fn new(id: i32) -> Self{
        return Self{id};
    }
}
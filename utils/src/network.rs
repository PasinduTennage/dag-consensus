use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub struct Network {
    pub id: i32,

    pub incoming_connections: HashMap<i32, TcpStream>,
    pub outgoing_connections: HashMap<i32, Arc<Mutex<TcpStream>>>,
}
impl Network {
    pub fn new(id: i32) -> Network {
        Self {
            id: id,
            incoming_connections: HashMap::new(),
            outgoing_connections: HashMap::new(),
        }
    }


}

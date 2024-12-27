mod network;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct ClientRequest{
    pub id: String,
    pub payload: Vec<u8>,
    pub sender: i32,
}


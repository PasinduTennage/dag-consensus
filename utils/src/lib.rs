mod network;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct ClientRequestBatch {
    pub id: String,
    pub payload: Vec<Vec<u8>>,
    pub sender: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlMessage {
    pub operation: i32,
    pub str_params :Vec<String>,
    pub int_params :Vec<i32>,
    pub sender: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReplicaMessage {
   pub sender: i32,

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    ClientRequest(ClientRequestBatch),
    Control(ControlMessage),
    ReplicaMessage(ReplicaMessage),
}
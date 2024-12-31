use serde::de::Error;
use serde::Deserialize;
use serde_json::Value::String;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct Peer {
    name: i32,
    address: String,
}

#[derive(Debug, Deserialize)]
struct Client {
    name: i32,
    address: String,
}

#[derive(Debug, Deserialize)]
struct NetworkConfig {
    peers: Vec<Peer>,
    clients: Vec<Client>,
}

impl NetworkConfig {
    pub fn load_config(file_path: &str) -> Result<NetworkConfig, Box<dyn Error>> {
        let yaml_content = fs::read_to_string(file_path)?;

        // Deserialize YAML into Config struct
        let network_config: NetworkConfig = serde_yaml::from_str(&yaml_content)?;
        Ok(network_config)
    }
}

pub struct Config {
    pub id: i32,
    pub node_type: String,
    pub listen_addr: String,
    pub remotes: HashMap<i32, String>,
}

impl Config {
    pub fn new(id: i32, node_type: String, net_config: NetworkConfig) -> Config {
        let mut listen_add = "";
        let mut remotes: HashMap<i32, String> = HashMap::new();

        if node_type == "peer" {
            for i in 0..net_config.peers.len() {
                if net_config.peers[i].name == id {
                    listen_add = &net_config.peers[i].address;
                }
                remotes.insert(net_config.peers[i].name, net_config.peers[i].address);
            }

            for i in 0..net_config.clients.len() {
                remotes.insert(net_config.clients[i].name, net_config.clients[i].address);
            }
        } else if node_type == "client" {
            for i in 0..net_config.peers.len() {
                remotes.insert(net_config.peers[i].name, net_config.peers[i].address);
            }

            for i in 0..net_config.clients.len() {
                if net_config.clients[i].name == id {
                    listen_add = &net_config.clients[i].address;
                }
            }
        }

        Self {
            id: id,
            node_type,
            listen_addr,
            remotes,
        }
    }
}
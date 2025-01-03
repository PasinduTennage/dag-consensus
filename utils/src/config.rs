// use serde::de::Error;
// use serde::Deserialize;
// use std::collections::HashMap;
// use std::fs;
// use std::string::String;
// use std::sync::Arc;
//
// #[derive(Debug, Deserialize)]
// struct Peer {
//     name: i32,
//     address: String,
// }
//
// #[derive(Debug, Deserialize)]
// struct Client {
//     name: i32,
//     address: String,
// }
//
// #[derive(Debug, Deserialize)]
// struct NetworkConfig {
//     peers: Vec<Peer>,
//     clients: Vec<Client>,
// }
//
// impl NetworkConfig {
//     pub fn load_config(file_path: &str) -> NetworkConfig {
//         let yaml_content = fs::read_to_string(file_path)?;
//
//         // Deserialize YAML into Config struct
//         let network_config: NetworkConfig = serde_yaml::from_str(&yaml_content)?;
//         network_config
//     }
// }
//
// pub struct Config {
//     pub id: i32,
//     pub node_type: String,
//     pub listen_addr: String,
//     pub remotes: HashMap<i32, String>,
// }
//
// impl Config {
//     pub fn new(id: i32, node_type: String, net_config: NetworkConfig) -> Config {
//         let mut listen = "";
//         let mut remotes_map: HashMap<i32, String> = HashMap::new();
//
//         if node_type == "peer" {
//             for i in 0..net_config.peers.len() {
//                 if net_config.peers[i].name == id {
//                     listen = &net_config.peers[i].address;
//                 }
//                 remotes_map.insert(net_config.peers[i].name, net_config.peers[i].address);
//             }
//
//             for i in 0..net_config.clients.len() {
//                 remotes_map.insert(net_config.clients[i].name, net_config.clients[i].address);
//             }
//         } else if node_type == "client" {
//             for i in 0..net_config.peers.len() {
//                 remotes_map.insert(net_config.peers[i].name, net_config.peers[i].address);
//             }
//
//             for i in 0..net_config.clients.len() {
//                 if net_config.clients[i].name == id {
//                     listen = &net_config.clients[i].address;
//                 }
//             }
//         }
//
//         if remotes_map.len() == 0 {
//             panic!("No network config found");
//         }
//
//         if listen == "" {
//             panic!("No listen address found");
//         }
//
//         Self {
//             id: id,
//             node_type,
//             listen_addr: listen.parse().unwrap(),
//             remotes: remotes_map,
//         }
//     }
// }

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Handshake {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: i32,
    #[serde(rename = "serverAddress")]
    pub server_address: String,
    #[serde(rename = "serverPort")]
    pub server_port: u16,
    #[serde(rename = "nextState")]
    pub next_state: i32,
}

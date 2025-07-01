use iron_oxide_protocol::packet;
use serde::{Deserialize, Serialize};

packet! {
    #[derive(Debug)]
    pub struct StatusRequest(0x00) {}
}

packet! {
    #[derive(Debug)]
    pub struct StatusResponse(0x00) {
        response: String,
    }
}

packet! {
    #[derive(Debug)]
    pub struct PingRequest(0x01) {
        payload: i64,
    }
}

packet! {
    #[derive(Debug)]
    pub struct PongResponse(0x01) {
        payload: i64,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub version: Version,
    pub players: Players,
    pub description: Description,
    #[serde(rename = "favicon")]
    pub favicon: Option<String>,
}

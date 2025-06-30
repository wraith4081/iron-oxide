use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    pub version: Version,
    pub players: Players,
    pub description: Description,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: bool,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Vec<Player>>,
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

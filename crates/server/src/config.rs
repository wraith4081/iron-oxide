use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
    pub players: Players,
}

#[derive(Deserialize)]
pub struct Server {
    pub address: String,
    pub motd: String,
}

#[derive(Deserialize)]
pub struct Players {
    pub max_players: i32,
}
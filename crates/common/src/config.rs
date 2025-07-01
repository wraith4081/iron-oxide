use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid view distance: {0}. Must be between 2 and 32.")]
    InvalidViewDistance(u8),
    #[error("Invalid simulation distance: {0}. Must be between 2 and 32.")]
    InvalidSimulationDistance(u8),
    #[error("Invalid max players: {0}. Must be a positive number.")]
    InvalidMaxPlayers(i32),
}

#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
    pub players: Players,
}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if !(2..=32).contains(&self.server.view_distance) {
            return Err(ConfigError::InvalidViewDistance(self.server.view_distance));
        }
        if !(2..=32).contains(&self.server.simulation_distance) {
            return Err(ConfigError::InvalidSimulationDistance(
                self.server.simulation_distance,
            ));
        }
        if self.players.max_players <= 0 {
            return Err(ConfigError::InvalidMaxPlayers(self.players.max_players));
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Server {
    pub address: String,
    pub motd: String,
    pub view_distance: u8,
    pub simulation_distance: u8,
}

#[derive(Deserialize)]
pub struct Players {
    pub max_players: i32,
}

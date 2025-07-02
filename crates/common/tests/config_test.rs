
use iron_oxide_common::config::{Config, ConfigError, Players, Server};

fn create_test_config(view_distance: u8, simulation_distance: u8, max_players: i32) -> Config {
    Config {
        server: Server {
            address: "127.0.0.1:25565".to_string(),
            motd: "A Minecraft Server".to_string(),
            view_distance,
            simulation_distance,
            enable_packet_logging: false
        },
        players: Players { max_players },
    }
}

#[test]
fn test_valid_config() {
    let config = create_test_config(10, 10, 20);
    assert!(config.validate().is_ok());
}

#[test]
fn test_invalid_view_distance_too_low() {
    let config = create_test_config(1, 10, 20);
    let result = config.validate();
    assert!(result.is_err());
    match result.unwrap_err() {
        ConfigError::InvalidViewDistance(v) => assert_eq!(v, 1),
        _ => panic!("Expected InvalidViewDistance error"),
    }
}

#[test]
fn test_invalid_view_distance_too_high() {
    let config = create_test_config(33, 10, 20);
    let result = config.validate();
    assert!(result.is_err());
    match result.unwrap_err() {
        ConfigError::InvalidViewDistance(v) => assert_eq!(v, 33),
        _ => panic!("Expected InvalidViewDistance error"),
    }
}

#[test]
fn test_invalid_simulation_distance_too_low() {
    let config = create_test_config(10, 1, 20);
    let result = config.validate();
    assert!(result.is_err());
    match result.unwrap_err() {
        ConfigError::InvalidSimulationDistance(v) => assert_eq!(v, 1),
        _ => panic!("Expected InvalidSimulationDistance error"),
    }
}

#[test]
fn test_invalid_simulation_distance_too_high() {
    let config = create_test_config(10, 33, 20);
    let result = config.validate();
    assert!(result.is_err());
    match result.unwrap_err() {
        ConfigError::InvalidSimulationDistance(v) => assert_eq!(v, 33),
        _ => panic!("Expected InvalidSimulationDistance error"),
    }
}

#[test]
fn test_invalid_max_players_zero() {
    let config = create_test_config(10, 10, 0);
    let result = config.validate();
    assert!(result.is_err());
    match result.unwrap_err() {
        ConfigError::InvalidMaxPlayers(v) => assert_eq!(v, 0),
        _ => panic!("Expected InvalidMaxPlayers error"),
    }
}

#[test]
fn test_invalid_max_players_negative() {
    let config = create_test_config(10, 10, -1);
    let result = config.validate();
    assert!(result.is_err());
    match result.unwrap_err() {
        ConfigError::InvalidMaxPlayers(v) => assert_eq!(v, -1),
        _ => panic!("Expected InvalidMaxPlayers error"),
    }
}

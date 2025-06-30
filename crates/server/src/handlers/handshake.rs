use anyhow::Result;
use iron_oxide_protocol::packet::handshake::Handshake;
use tracing::info;
use crate::connection::{Connection, ConnectionState};

pub async fn handle_handshake(conn: &mut Connection) -> Result<ConnectionState> {
    let handshake: Handshake = conn.read_packet().await?.unwrap();

    info!("Handshake successful. Next state: {:?}", handshake.next_state);

    match handshake.next_state {
        1 => Ok(ConnectionState::Status),
        2 => Ok(ConnectionState::Login),
        _ => {
            anyhow::bail!("Invalid next state: {}", handshake.next_state);
        }
    }
}


use anyhow::Result;
use iron_oxide_versions::v1_20_6::packets::handshake::Handshake;
use iron_oxide_versions::VersionManager;
use tracing::info;
use crate::connection::{Connection, ConnectionState};

pub async fn handle_handshake(conn: &mut Connection) -> Result<ConnectionState> {
    let handshake: Handshake = conn.read_packet().await?.unwrap();

    conn.protocol_version = handshake.protocol_version;

    if let Err(e) = VersionManager::get_version(handshake.protocol_version) {
        info!("Unsupported protocol version: {}", handshake.protocol_version);
        // TODO: send disconnect packet
        return Err(e.into());
    }

    info!(
        "Handshake successful. Protocol version: {}, Next state: {:?}",
        handshake.protocol_version,
        handshake.next_state
    );

    match handshake.next_state {
        1 => Ok(ConnectionState::Status),
        2 => Ok(ConnectionState::Login),
        _ => {
            anyhow::bail!("Invalid next state: {}", handshake.next_state);
        }
    }
}


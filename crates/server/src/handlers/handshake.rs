use iron_oxide_common::connection::{Connection, ConnectionState};
use iron_oxide_protocol::error::{Error, Result};
use iron_oxide_versions::v1_20_6::packets::handshake::Handshake;
use iron_oxide_versions::VersionManager;
use tracing::info;

pub async fn handle_handshake(conn: &mut Connection) -> Result<ConnectionState> {
    let handshake: Handshake = conn.read_packet().await?.ok_or_else(|| Error::Protocol("Handshake packet not received".to_string()))?;

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
            Err(Error::Protocol(format!("Invalid next state: {}", handshake.next_state)))
        }
    }
}
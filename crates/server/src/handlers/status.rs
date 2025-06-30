use std::io;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{error, info};
use crate::config::Config;
use crate::network;
use protocol::packet::status::{Description, Players, StatusResponse, Version};

pub async fn handle_status(stream: &mut TcpStream, config: Arc<Config>) -> io::Result<()> {
    let _packet_length = network::read_varint(stream).await?;
    let packet_id = network::read_varint(stream).await?;

    if packet_id == 0x00 {
        handle_status_request(stream, config).await?;
    } else if packet_id == 0x01 {
        handle_ping_request(stream).await?;
    } else {
        error!("Invalid packet ID for Status: {}", packet_id);
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid packet ID for Status",
        ));
    }

    Ok(())
}

async fn handle_status_request(stream: &mut TcpStream, config: Arc<Config>) -> io::Result<()> {
    info!("Handling Status Request");
    let response = StatusResponse {
        version: Version {
            name: "1.20.6".to_string(),
            protocol: 766,
        },
        players: Players {
            max: config.players.max_players,
            online: 0,
            sample: None,
        },
        description: Description {
            text: config.server.motd.clone(),
        },
        favicon: None,
        enforces_secure_chat: false,
    };

    let response_json = match serde_json::to_string(&response) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to serialize status response: {}", e);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to serialize status response",
            ));
        }
    };

    let mut packet = Vec::new();
    network::write_varint_to_vec(&mut packet, 0x00)?; // Packet ID
    network::write_string_to_vec(&mut packet, &response_json)?;

    network::write_varint(stream, packet.len() as i32).await?;
    stream.write_all(&packet).await?;
    info!("Sent Status Response");
    Ok(())
}

async fn handle_ping_request(stream: &mut TcpStream) -> io::Result<()> {
    info!("Handling Ping Request");
    let payload = stream.read_i64().await?;
    info!("Ping Request payload: {}", payload);

    let mut packet = Vec::new();
    packet.push(0x01); // Packet ID
    packet.extend(&payload.to_be_bytes());

    network::write_varint(stream, packet.len() as i32).await?;
    stream.write_all(&packet).await?;
    info!("Sent Pong Response");
    Ok(())
}

use std::io;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tracing::{error, info};
use crate::connection::State;
use crate::network;

pub async fn handle_handshake(stream: &mut TcpStream) -> io::Result<State> {
    let _packet_length = network::read_varint(stream).await?;
    let packet_id = network::read_varint(stream).await?;

    if packet_id != 0x00 {
        error!("Invalid packet ID for Handshake");
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid packet ID for Handshake",
        ));
    }

    let _protocol_version = network::read_varint(stream).await?;
    let server_address_length = network::read_varint(stream).await?;
    let mut server_address_bytes = vec![0; server_address_length as usize];
    stream.read_exact(&mut server_address_bytes).await?;
    let _server_address = String::from_utf8_lossy(&server_address_bytes);
    let _server_port = stream.read_u16().await?;
    let next_state = network::read_varint(stream).await?;

    info!("Handshake successful. Next state: {}", next_state);

    match next_state {
        1 => Ok(State::Status),
        2 => Ok(State::Login),
        _ => {
            error!("Invalid next state: {}", next_state);
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid next state",
            ))
        }
    }
}

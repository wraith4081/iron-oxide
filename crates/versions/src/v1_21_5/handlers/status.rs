use tracing::info;
use iron_oxide_protocol::error::{Error, Result};
use iron_oxide_protocol::stream::ConnectionIO;
use crate::v1_21_5::packets::status::{
    PingRequest, PongResponse, StatusRequest, StatusResponse as StatusResponsePacket,
};
use crate::v1_21_5::packets::status::{Description, Players, Version};

pub async fn handle_status(
    conn: &mut (impl ConnectionIO + Send),
    max_players: i32,
    motd: String,
) -> Result<()> {
    let _: StatusRequest = conn.read_packet_io().await?.ok_or_else(|| Error::Protocol("StatusRequest packet not received".to_string()))?;
    info!("Handling Status Request");

    let response = StatusResponsePacket {
        response: serde_json::to_string(&StatusResponse {
            version: Version {
                name: "1.21.5".to_string(),
                protocol: 770,
            },
            players: Players {
                max: max_players,
                online: 0,
                sample: vec![],
            },
            description: Description {
                text: motd,
            },
            favicon: None,
        }).map_err(|e| Error::PacketSerialization(e.to_string()))?,
    };
    conn.write_packet_io(response).await?;
    info!("Sent Status Response");

    if let Some(ping) = conn.read_packet_io::<PingRequest>().await? {
        info!("Handling Ping Request, payload: {}", ping.payload);

        let pong = PongResponse {
            payload: ping.payload,
        };
        conn.write_packet_io(pong).await?;
        info!("Sent Pong Response");
    } else {
        info!("Client disconnected after status response");
    }

    Ok(())
}

#[derive(serde::Serialize)]
struct StatusResponse {
    version: Version,
    players: Players,
    description: Description,
    #[serde(skip_serializing_if = "Option::is_none")]
    favicon: Option<String>,
}
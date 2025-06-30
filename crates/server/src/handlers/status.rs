use anyhow::Result;
use iron_oxide_protocol::packet::status::{
    PingRequest, PongResponse, StatusRequest, StatusResponse as StatusResponsePacket,
};
use iron_oxide_protocol::packet::status::{Description, Players, Version};
use tracing::info;
use crate::connection::Connection;

pub async fn handle_status(conn: &mut Connection) -> Result<()> {
    let _: StatusRequest = conn.read_packet().await?.unwrap();
    info!("Handling Status Request");

    let response = StatusResponsePacket {
        response: serde_json::to_string(&StatusResponse {
            version: Version {
                name: "1.20.6".to_string(),
                protocol: 766,
            },
            players: Players {
                max: conn.config.players.max_players,
                online: 0,
                sample: vec![],
            },
            description: Description {
                text: conn.config.server.motd.clone(),
            },
            favicon: None,
        })?,
    };
    conn.write_packet(response).await?;
    info!("Sent Status Response");

    if let Some(ping) = conn.read_packet::<PingRequest>().await? {
        info!("Handling Ping Request, payload: {}", ping.payload);

        let pong = PongResponse {
            payload: ping.payload,
        };
        conn.write_packet(pong).await?;
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


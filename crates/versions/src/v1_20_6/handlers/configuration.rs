use anyhow::Result;
use std::sync::Arc;
use tracing::info;
use iron_oxide_common::config::Config;
use iron_oxide_common::connection::Connection;
use crate::v1_20_6::packets::configuration::{
    ClientboundPluginMessage, ClientInformation, FinishConfiguration, AcknowledgeFinishConfiguration,
};

pub async fn handle_configuration(conn: &mut Connection, _config: Arc<Config>) -> Result<()> {
    info!("Client entered configuration state");

    // 1. Receive Client Information
    let client_info: ClientInformation = conn.read_packet().await?.unwrap();
    info!("Received client information: {:?}", client_info);

    // 2. Send minecraft:brand
    let brand_message = ClientboundPluginMessage {
        channel: "minecraft:brand".to_string(),
        data: vec![0x09, b'I', b'r', b'o', b'n', b'O', b'x', b'i', b'd', b'e'], // "IronOxide"
    };
    conn.write_packet(brand_message).await?;
    info!("Sent minecraft:brand");

    // 3. Send Finish Configuration
    conn.write_packet(FinishConfiguration).await?;
    info!("Sent Finish Configuration");

    // 4. Wait for Acknowledge Finish Configuration
    let _ack: AcknowledgeFinishConfiguration = conn.read_packet().await?.unwrap();
    info!("Received Acknowledge Finish Configuration");

    Ok(())
}

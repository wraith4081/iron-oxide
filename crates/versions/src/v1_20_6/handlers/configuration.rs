use anyhow::Result;
use std::sync::Arc;
use tracing::info;
use iron_oxide_common::config::Config;
use iron_oxide_common::connection::Connection;
use crate::v1_20_6::packets::configuration::{
    ClientboundPluginMessage, ClientInformation, FinishConfiguration, AcknowledgeFinishConfiguration,
    ClientboundKnownPacks, KnownPack, ServerboundKnownPacks, RegistryData, ServerboundPluginMessage,
};
use std::fs;
use fastnbt::Value;
use serde_json::Value as JsonValue;
use iron_oxide_protocol::packet::data::read_varint;

fn json_to_nbt(json: &JsonValue) -> Value {
    match json {
        JsonValue::Null => Value::Compound(Default::default()),
        JsonValue::Bool(b) => Value::Byte(if *b { 1 } else { 0 }),
        JsonValue::Number(n) => {
            if n.is_f64() {
                Value::Double(n.as_f64().unwrap())
            } else {
                Value::Long(n.as_i64().unwrap())
            }
        }
        JsonValue::String(s) => Value::String(s.clone()),
        JsonValue::Array(a) => Value::List(a.iter().map(json_to_nbt).collect()),
        JsonValue::Object(o) => Value::Compound(
            o.iter()
                .map(|(k, v)| (k.clone(), json_to_nbt(v)))
                .collect(),
        ),
    }
}

pub async fn handle_configuration(conn: &mut Connection, _config: Arc<Config>) -> Result<()> {
    info!("Client entered configuration state");

    // The client must send Client Information first.
    let client_info: ClientInformation = conn.read_packet().await?.unwrap();
    info!("Received client information: {:?}", client_info);

    // The client may send plugin messages. We need to handle them.
    loop {
        let packet_id = read_varint(&mut conn.peek_packet().await?)?;
        match packet_id {
            0x00 => { // Client Information
                let _client_info: ClientInformation = conn.read_packet().await?.unwrap();
                info!("Received client information: {:?}", _client_info);
            }
            0x02 => { // Plugin Message
                let plugin_message: ServerboundPluginMessage = conn.read_packet().await?.unwrap();
                info!("Received plugin message: {:?}", plugin_message);
            }
            0x07 => { // Known Packs
                let serverbound_known_packs: ServerboundKnownPacks = conn.read_packet().await?.unwrap();
                info!("Received Known Packs: {:?}", serverbound_known_packs);
                break;
            }
            _ => {
                // For now, we'll ignore other packets.
                let _ = conn.read_packet_raw().await?;
            }
        }
    }

    // Send server configuration
    send_server_configuration(conn).await?;

    // Wait for Acknowledge Finish Configuration
    let _ack: AcknowledgeFinishConfiguration = conn.read_packet().await?.unwrap();
    info!("Received Acknowledge Finish Configuration");

    Ok(())
}

async fn send_server_configuration(conn: &mut Connection) -> Result<()> {
    // Send minecraft:brand
    let brand_message = ClientboundPluginMessage {
        channel: "minecraft:brand".to_string(),
        data: vec![0x09, b'I', b'r', b'o', b'n', b'O', b'x', b'i', b'd', b'e'], // "IronOxide"
    };
    conn.write_packet(brand_message).await?;
    info!("Sent minecraft:brand");

    // Send Known Packs
    let known_packs = ClientboundKnownPacks {
        packs: vec![KnownPack {
            namespace: "minecraft".to_string(),
            id: "core".to_string(),
            version: "1.20.6".to_string(),
        }],
    };
    conn.write_packet(known_packs).await?;
    info!("Sent Known Packs");

    // Send Registry Data
    let registry_data_str = fs::read_to_string("docs/registry-data.json")?;
    let registry_data_json: JsonValue = serde_json::from_str(&registry_data_str)?;

    if let JsonValue::Object(registries) = registry_data_json {
        for (registry_id, registry) in registries {
            if let JsonValue::Object(entries) = registry {
                let mut nbt_entries = Vec::new();
                for (entry_id, data) in entries {
                    let nbt_value = json_to_nbt(&data);
                    nbt_entries.push((entry_id, Some(nbt_value)));
                }

                let entries_for_packet: Vec<(String, Option<&Value>)> = nbt_entries.iter().map(|(k, v)| (k.clone(), v.as_ref())).collect();

                let registry_data_packet = RegistryData {
                    registry_id,
                    entries: entries_for_packet,
                };
                conn.write_packet(registry_data_packet).await?;
            }
        }
    }
    info!("Sent Registry Data");

    // Send Finish Configuration
    conn.write_packet(FinishConfiguration).await?;
    info!("Sent Finish Configuration");

    Ok(())
}

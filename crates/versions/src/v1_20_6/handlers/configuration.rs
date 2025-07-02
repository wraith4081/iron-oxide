use std::sync::Arc;
use tracing::info;
use iron_oxide_common::config::Config;
use iron_oxide_common::connection::{Connection, ConnectionState};
use crate::v1_20_6::packets::configuration::{
    ClientboundPluginMessage, ClientInformation, FinishConfiguration, AcknowledgeFinishConfiguration,
    ClientboundKnownPacks, KnownPack, ServerboundKnownPacks, RegistryData, ServerboundPluginMessage,
};
use std::fs;
use fastnbt::Value;
use serde_json::Value as JsonValue;
use iron_oxide_protocol::error::{Error, Result};
use iron_oxide_protocol::packet::raw_data::read_varint;
use iron_oxide_protocol::packet::types::PacketBytes;

fn json_to_nbt(json: &JsonValue) -> Result<Value> {
    match json {
        JsonValue::Null => Ok(Value::Compound(Default::default())),
        JsonValue::Bool(b) => Ok(Value::Byte(if *b { 1 } else { 0 })),
        JsonValue::Number(n) => {
            if n.is_f64() {
                Ok(Value::Double(n.as_f64().ok_or_else(|| Error::Protocol("Invalid f64 in JSON".to_string()))?))
            } else {
                Ok(Value::Long(n.as_i64().ok_or_else(|| Error::Protocol("Invalid i64 in JSON".to_string()))?))
            }
        }
        JsonValue::String(s) => Ok(Value::String(s.clone())),
        JsonValue::Array(a) => {
            let mut list = Vec::new();
            for item in a {
                list.push(json_to_nbt(item)?);
            }
            Ok(Value::List(list))
        }
        JsonValue::Object(o) => {
            let mut map = std::collections::HashMap::new();
            for (k, v) in o {
                map.insert(k.clone(), json_to_nbt(v)?);
            }
            Ok(Value::Compound(map))
        }
    }
}

pub async fn handle_configuration(conn: &mut Connection, _config: Arc<Config>) -> Result<()> {
    info!("Client entered configuration state");

    loop {
        let packet_id = read_varint(&mut conn.peek_packet().await?)?;
        match packet_id {
            0x0e => { // Client Information
                let _client_info: ClientInformation = conn.read_packet().await?.ok_or_else(|| Error::Protocol("ClientInformation packet not received".to_string()))?;
                info!("Received client information: {:?}", _client_info);
                send_initial_server_configuration(conn).await?
            }
            0x19 => { // Plugin Message
                let plugin_message: ServerboundPluginMessage = conn.read_packet().await?.ok_or_else(|| Error::Protocol("ServerboundPluginMessage packet not received".to_string()))?;
                info!("Received plugin message: {:?}", plugin_message);
            }
            0x07 => { // Known Packs
                let serverbound_known_packs: ServerboundKnownPacks = conn.read_packet().await?.ok_or_else(|| Error::Protocol("ServerboundKnownPacks packet not received".to_string()))?;
                info!("Received Known Packs: {:?}", serverbound_known_packs);
                send_final_server_configuration(conn).await?;
            }
            0x18 => {
                let _ack: AcknowledgeFinishConfiguration = conn.read_packet().await?.ok_or_else(|| Error::Protocol("AcknowledgeFinishConfiguration packet not received".to_string()))?;
                info!("Received Acknowledge Finish Configuration");
                return Ok(())
            }
            _ => {
                info!("Unknown packet {}", packet_id);
                // For now, we'll ignore other packets.
                let _ = conn.read_packet_raw().await?;
            }
        }
    }
}

async fn send_initial_server_configuration(conn: &mut Connection) -> Result<()> {
    let brand_message = ClientboundPluginMessage {
        channel: "minecraft:brand".to_string(),
        data: PacketBytes(vec![0x09, b'I', b'r', b'o', b'n', b'O', b'x', b'i', b'd', b'e']), // "IronOxide"
    };
    conn.write_packet(brand_message).await?;
    info!("Sent minecraft:brand");

    // Send Feature Flags
    let feature_flags_packet = crate::v1_20_6::packets::configuration::FeatureFlags {
        feature_flags: vec!["minecraft:vanilla".to_string()],
    };
    conn.write_packet(feature_flags_packet).await?;
    info!("Sent Feature Flags");

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
    
    Ok(())
}

async fn send_final_server_configuration(conn: &mut Connection) -> Result<()> {
    // Send Registry Data
    let registry_data_str = fs::read_to_string("config/v1_20_6/registry-data.json")?;
    let registry_data_json: JsonValue = serde_json::from_str(&registry_data_str).map_err(|e| Error::Protocol(format!("Failed to parse registry data: {}", e)))?;

    if let JsonValue::Object(registries) = registry_data_json {
        for (registry_id, registry) in registries {
            if let JsonValue::Object(entries) = registry {
                let mut nbt_entries = Vec::new();
                for (entry_id, data) in entries {
                    let nbt_value = json_to_nbt(&data)?;
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

    // Send Update Tags
    let update_tags_packet = crate::v1_20_6::packets::configuration::UpdateTags {
        tags: load_tags()?,
    };
    conn.write_packet(update_tags_packet).await?;
    info!("Sent Update Tags");

    // Send Finish Configuration
    conn.write_packet(FinishConfiguration {}).await?;
    info!("Sent Finish Configuration");

    Ok(())
}

fn load_tags() -> Result<Vec<(String, Vec<(String, Vec<i32>)>)>> {
    let tags_str = fs::read_to_string("config/v1_20_6/tags.json")?;
    let tags_json: JsonValue = serde_json::from_str(&tags_str).map_err(|e| Error::Protocol(format!("Failed to parse tags: {}", e)))?;
    let mut tags = Vec::new();

    if let JsonValue::Object(registries) = tags_json {
        for (registry_id, registry_tags) in registries {
            let mut tag_list = Vec::new();
            if let JsonValue::Object(tags_map) = registry_tags {
                for (tag_name, entries) in tags_map {
                    let mut entry_list = Vec::new();
                    if let JsonValue::Array(entry_array) = entries {
                        for entry in entry_array {
                            if let JsonValue::Number(n) = entry {
                                entry_list.push(n.as_i64().ok_or_else(|| Error::Protocol("Invalid i64 in tags".to_string()))? as i32);
                            }
                        }
                    }
                    tag_list.push((tag_name, entry_list));
                }
            }
            tags.push((registry_id, tag_list));
        }
    }

    Ok(tags)
}
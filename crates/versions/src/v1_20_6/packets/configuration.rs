use iron_oxide_protocol::packet::{Packet, PacketReadError, PacketWriteError};
use iron_oxide_protocol::packet::data::{read_string, write_string, write_varint, read_varint, read_bytes};
use fastnbt::Value;

// Clientbound packets
#[derive(Debug)]
pub struct ClientboundPluginMessage {
    pub channel: String,
    pub data: Vec<u8>,
}

impl Packet for ClientboundPluginMessage {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        unimplemented!()
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        write_varint(buffer, 0x01)?;
        write_string(buffer, &self.channel)?;
        buffer.extend_from_slice(&self.data);
        Ok(())
    }
}

#[derive(Debug)]
pub struct FinishConfiguration;

impl Packet for FinishConfiguration {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        unimplemented!()
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        write_varint(buffer, 0x03)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

#[derive(Debug)]
pub struct ClientboundKnownPacks {
    pub packs: Vec<KnownPack>,
}

impl Packet for ClientboundKnownPacks {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        unimplemented!()
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        write_varint(buffer, 0x0E)?;
        write_varint(buffer, self.packs.len() as i32)?;
        for pack in &self.packs {
            write_string(buffer, &pack.namespace)?;
            write_string(buffer, &pack.id)?;
            write_string(buffer, &pack.version)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct RegistryData<'a> {
    pub registry_id: String,
    pub entries: Vec<(String, Option<&'a Value>)>,
}

impl Packet for RegistryData<'_> {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        unimplemented!()
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        write_varint(buffer, 0x07)?;
        write_string(buffer, &self.registry_id)?;
        write_varint(buffer, self.entries.len() as i32)?;
        for (entry_id, data) in &self.entries {
            write_string(buffer, entry_id)?;
            if let Some(nbt) = data {
                let mut nbt_buf = Vec::new();
                fastnbt::to_writer(&mut nbt_buf, nbt).unwrap();
                write_varint(buffer, 1i32)?;
                buffer.extend_from_slice(&nbt_buf);
            } else {
                write_varint(buffer, 0)?;
            }
        }
        Ok(())
    }
}

// Serverbound packets
#[derive(Debug)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: i32,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}

impl Packet for ClientInformation {
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketReadError> {
        let locale = read_string(buffer)?;
        let view_distance = read_bytes(buffer, 1)?[0];
        let chat_mode = read_varint(buffer)?;
        let chat_colors = read_bytes(buffer, 1)?[0] != 0;
        let displayed_skin_parts = read_bytes(buffer, 1)?[0];
        let main_hand = read_varint(buffer)?;
        let enable_text_filtering = read_bytes(buffer, 1)?[0] != 0;
        let allow_server_listings = read_bytes(buffer, 1)?[0] != 0;

        Ok(Self {
            locale,
            view_distance,
            chat_mode,
            chat_colors,
            displayed_skin_parts,
            main_hand,
            enable_text_filtering,
            allow_server_listings,
        })
    }

    fn write(&self, _: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ServerboundPluginMessage {
    pub channel: String,
    pub data: Vec<u8>,
}

impl Packet for ServerboundPluginMessage {
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketReadError> {
        let channel = read_string(buffer)?;
        let data = buffer.to_vec();
        *buffer = &buffer[buffer.len()..];
        Ok(Self { channel, data })
    }

    fn write(&self, _: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct AcknowledgeFinishConfiguration;

impl Packet for AcknowledgeFinishConfiguration {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        Ok(Self)
    }

    fn write(&self, _: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct ServerboundKnownPacks {
    pub packs: Vec<KnownPack>,
}

impl Packet for ServerboundKnownPacks {
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketReadError> {
        let size = read_varint(buffer)?;
        let mut packs = Vec::with_capacity(size as usize);
        for _ in 0..size {
            packs.push(KnownPack {
                namespace: read_string(buffer)?,
                id: read_string(buffer)?,
                version: read_string(buffer)?,
            });
        }
        Ok(Self { packs })
    }

    fn write(&self, _: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct FeatureFlags {
    pub feature_flags: Vec<String>,
}

impl Packet for FeatureFlags {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        unimplemented!()
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        write_varint(buffer, 0x0C)?;
        write_varint(buffer, self.feature_flags.len() as i32)?;
        for flag in &self.feature_flags {
            write_string(buffer, flag)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct UpdateTags {
    pub tags: Vec<(String, Vec<(String, Vec<i32>)>)>,
}

impl Packet for UpdateTags {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        unimplemented!()
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        write_varint(buffer, 0x0D)?;
        write_varint(buffer, self.tags.len() as i32)?;
        for (registry, tags) in &self.tags {
            write_string(buffer, registry)?;
            write_varint(buffer, tags.len() as i32)?;
            for (name, entries) in tags {
                write_string(buffer, name)?;
                write_varint(buffer, entries.len() as i32)?;
                for entry in entries {
                    write_varint(buffer, *entry)?;
                }
            }
        }
        Ok(())
    }
}

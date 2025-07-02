use iron_oxide_protocol::error::{Error, Result};
use iron_oxide_protocol::packet;
use fastnbt::Value;
use iron_oxide_protocol::packet::types::{PacketByte, PacketBytes};

packet! {
    #[derive(Debug)]
    pub struct ClientboundPluginMessage(0x01) {
        channel: String,
        data: PacketBytes,
    }
}

packet! {
    #[derive(Debug)]
    pub struct FinishConfiguration(0x03) {}
}

#[derive(Debug, Clone)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

impl packet::data::PacketData for KnownPack {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        Ok(Self {
            namespace: packet::data::PacketData::read(buffer)?,
            id: packet::data::PacketData::read(buffer)?,
            version: packet::data::PacketData::read(buffer)?,
        })
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        packet::data::PacketData::write(&self.namespace, buffer)?;
        packet::data::PacketData::write(&self.id, buffer)?;
        packet::data::PacketData::write(&self.version, buffer)?;
        Ok(())
    }
}

fn read_known_packs(buffer: &mut &[u8]) -> Result<Vec<KnownPack>> {
    let len: i32 = packet::data::PacketData::read(buffer)?;
    let mut packs = Vec::with_capacity(len as usize);
    for _ in 0..len {
        packs.push(packet::data::PacketData::read(buffer)?);
    }
    Ok(packs)
}

fn write_known_packs(packs: &Vec<KnownPack>, buffer: &mut Vec<u8>) -> Result<()> {
    packet::data::PacketData::write(&(packs.len() as i32), buffer)?;
    for pack in packs {
        packet::data::PacketData::write(pack, buffer)?;
    }
    Ok(())
}

packet! {
    #[derive(Debug)]
    pub struct ClientboundKnownPacks(0x0E) {
        packs: Vec<KnownPack> = (read_known_packs, write_known_packs),
    }
}

pub struct RegistryData<'a> {
    pub registry_id: String,
    pub entries: Vec<(String, Option<&'a Value>)>,
}

// This packet is too complex for the macro, so we'll implement it manually.
impl packet::Packet for RegistryData<'_> {
    fn read(_: &mut &[u8]) -> Result<Self> {
        unimplemented!()
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        packet::raw_data::write_varint(buffer, 0x07)?;
        packet::data::PacketData::write(&self.registry_id, buffer)?;
        packet::data::PacketData::write(&(self.entries.len() as i32), buffer)?;
        for (entry_id, data) in &self.entries {
            packet::data::PacketData::write(entry_id, buffer)?;
            if let Some(nbt) = data {
                let mut nbt_buf = Vec::new();
                fastnbt::to_writer(&mut nbt_buf, nbt).map_err(|e| Error::PacketSerialization(e.to_string()))?;
                packet::data::PacketData::write(&true, buffer)?;
                buffer.extend_from_slice(&nbt_buf);
            } else {
                packet::data::PacketData::write(&false, buffer)?;
            }
        }
        Ok(())
    }
}

packet! {
    #[derive(Debug)]
    pub struct ClientInformation(0x00) {
        locale: String,
        view_distance: PacketByte,
        chat_mode: i32,
        chat_colors: bool,
        displayed_skin_parts: PacketByte,
        main_hand: i32,
        enable_text_filtering: bool,
        allow_server_listings: bool,
    }
}

packet! {
    #[derive(Debug)]
    pub struct ServerboundPluginMessage(0x02) {
        channel: String,
        data: PacketBytes,
    }
}

packet! {
    #[derive(Debug)]
    pub struct AcknowledgeFinishConfiguration(0x03) {}
}

packet! {
    #[derive(Debug)]
    pub struct ServerboundKnownPacks(0x07) {
        packs: Vec<KnownPack> = (read_known_packs, write_known_packs),
    }
}

fn read_feature_flags(buffer: &mut &[u8]) -> Result<Vec<String>> {
    let len: i32 = packet::data::PacketData::read(buffer)?;
    let mut flags = Vec::with_capacity(len as usize);
    for _ in 0..len {
        flags.push(packet::data::PacketData::read(buffer)?);
    }
    Ok(flags)
}

fn write_feature_flags(flags: &Vec<String>, buffer: &mut Vec<u8>) -> Result<()> {
    packet::data::PacketData::write(&(flags.len() as i32), buffer)?;
    for flag in flags {
        packet::data::PacketData::write(flag, buffer)?;
    }
    Ok(())
}

packet! {
    #[derive(Debug)]
    pub struct FeatureFlags(0x0C) {
        feature_flags: Vec<String> = (read_feature_flags, write_feature_flags),
    }
}

type Tag = (String, Vec<i32>);
type TagRegistry = (String, Vec<Tag>);

fn read_tags(buffer: &mut &[u8]) -> Result<Vec<TagRegistry>> {
    let len: i32 = packet::data::PacketData::read(buffer)?;
    let mut registries = Vec::with_capacity(len as usize);
    for _ in 0..len {
        let registry_id = packet::data::PacketData::read(buffer)?;
        let tags_len: i32 = packet::data::PacketData::read(buffer)?;
        let mut tags = Vec::with_capacity(tags_len as usize);
        for _ in 0..tags_len {
            let tag_name = packet::data::PacketData::read(buffer)?;
            let entries_len: i32 = packet::data::PacketData::read(buffer)?;
            let mut entries = Vec::with_capacity(entries_len as usize);
            for _ in 0..entries_len {
                entries.push(packet::data::PacketData::read(buffer)?);
            }
            tags.push((tag_name, entries));
        }
        registries.push((registry_id, tags));
    }
    Ok(registries)
}

fn write_tags(registries: &Vec<TagRegistry>, buffer: &mut Vec<u8>) -> Result<()> {
    packet::data::PacketData::write(&(registries.len() as i32), buffer)?;
    for (registry_id, tags) in registries {
        packet::data::PacketData::write(registry_id, buffer)?;
        packet::data::PacketData::write(&(tags.len() as i32), buffer)?;
        for (tag_name, entries) in tags {
            packet::data::PacketData::write(tag_name, buffer)?;
            packet::data::PacketData::write(&(entries.len() as i32), buffer)?;
            for entry in entries {
                packet::data::PacketData::write(entry, buffer)?;
            }
        }
    }
    Ok(())
}

packet! {
    #[derive(Debug)]
    pub struct UpdateTags(0x0D) {
        tags: Vec<TagRegistry> = (read_tags, write_tags),
    }
}
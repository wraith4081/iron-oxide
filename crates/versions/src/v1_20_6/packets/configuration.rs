use iron_oxide_protocol::packet::{Packet, PacketReadError, PacketWriteError};
use iron_oxide_protocol::packet::data::{read_string, write_string, write_varint, read_varint};

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
        write_varint(buffer, 0x02)?;
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
        let view_distance = buffer[0];
        *buffer = &buffer[1..];
        let chat_mode = read_varint(buffer)?;
        let chat_colors = buffer[0] != 0;
        *buffer = &buffer[1..];
        let displayed_skin_parts = buffer[0];
        *buffer = &buffer[1..];
        let main_hand = read_varint(buffer)?;
        let enable_text_filtering = buffer[0] != 0;
        *buffer = &buffer[1..];
        let allow_server_listings = buffer[0] != 0;
        *buffer = &buffer[1..];

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
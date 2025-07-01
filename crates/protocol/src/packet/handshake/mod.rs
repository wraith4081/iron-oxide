use crate::error::Result;
use crate::packet::data::{read_string, read_unsigned_short, read_varint, write_string, write_unsigned_short, write_varint};
use crate::packet::Packet;

pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32,
}

impl Packet for Handshake {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        let protocol_version = read_varint(buffer)?;
        let server_address = read_string(buffer)?;
        let server_port = read_unsigned_short(buffer)?;
        let next_state = read_varint(buffer)?;
        Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        })
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        write_varint(buffer, self.protocol_version)?;
        write_string(buffer, &self.server_address)?;
        write_unsigned_short(buffer, self.server_port)?;
        write_varint(buffer, self.next_state)?;
        Ok(())
    }
}
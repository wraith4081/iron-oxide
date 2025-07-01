use crate::error::Result;
use crate::packet::data::PacketData;
use std::io::Read;

#[derive(Debug)]
pub struct PacketBytes(pub Vec<u8>);

impl PacketData for PacketBytes {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        let data = buffer.to_vec();
        *buffer = &buffer[buffer.len()..];
        Ok(PacketBytes(data))
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.extend_from_slice(&self.0);
        Ok(())
    }
}

#[derive(Debug)]
pub struct PacketByte(pub u8);

impl PacketData for PacketByte {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        let mut buf = [0];
        buffer.read_exact(&mut buf)?;
        Ok(PacketByte(buf[0]))
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(self.0);
        Ok(())
    }
}
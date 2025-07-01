use crate::error::Result;
use std::io::Read;
use uuid::Uuid;

pub trait PacketData: Sized {
    fn read(buffer: &mut &[u8]) -> Result<Self>;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<()>;
}

impl PacketData for String {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        super::raw_data::read_string(buffer)
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        super::raw_data::write_string(buffer, self)
    }
}

impl PacketData for u16 {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        super::raw_data::read_unsigned_short(buffer)
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        super::raw_data::write_unsigned_short(buffer, *self)
    }
}

impl PacketData for i32 {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        super::raw_data::read_varint(buffer)
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        super::raw_data::write_varint(buffer, *self)
    }
}

impl PacketData for i64 {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        super::raw_data::read_long(buffer)
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        super::raw_data::write_long(buffer, *self)
    }
}

impl PacketData for Uuid {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        super::raw_data::read_uuid(buffer)
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        super::raw_data::write_uuid(buffer, *self)
    }
}

impl PacketData for bool {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        let mut buf = [0];
        buffer.read_exact(&mut buf)?;
        Ok(buf[0] != 0)
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(if *self { 1 } else { 0 });
        Ok(())
    }
}

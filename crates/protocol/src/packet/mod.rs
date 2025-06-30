pub mod handshake;
pub mod status;
pub mod login;

use std::io::{Read, Write};
use thiserror::Error;

pub trait Packet {
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketReadError> where Self: Sized;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError>;
}

#[derive(Debug, Error)]
pub enum PacketReadError {
    #[error("Failed to read packet data")]
    IO(#[from] std::io::Error),
    #[error("Invalid packet data: {0}")]
    Invalid(String),
}

#[derive(Debug, Error)]
pub enum PacketWriteError {
    #[error("Failed to write packet data")]
    IO(#[from] std::io::Error),
}

pub mod data {
    use std::io::{Read, Write};
    use uuid::Uuid;
    use super::{PacketReadError, PacketWriteError};

    pub fn read_string(buffer: &mut &[u8]) -> Result<String, PacketReadError> {
        let len = read_varint(buffer)?;
        let mut str_buf = vec![0; len as usize];
        buffer.read_exact(&mut str_buf)?;
        Ok(String::from_utf8(str_buf).unwrap())
    }

    pub fn write_string(buffer: &mut Vec<u8>, value: &str) -> Result<(), PacketWriteError> {
        write_varint(buffer, value.len() as i32)?;
        buffer.write_all(value.as_bytes())?;
        Ok(())
    }

    pub fn read_varint(buffer: &mut &[u8]) -> Result<i32, PacketReadError> {
        let mut num_read = 0;
        let mut result = 0;
        let mut read;
        loop {
            let mut buf = [0];
            buffer.read_exact(&mut buf)?;
            read = buf[0];
            let value = (read & 0b0111_1111) as i32;
            result |= value << (7 * num_read);
            num_read += 1;
            if num_read > 5 {
                return Err(PacketReadError::Invalid("VarInt is too big".to_string()));
            }
            if (read & 0b1000_0000) == 0 {
                break;
            }
        }
        Ok(result)
    }

    pub fn write_varint(buffer: &mut Vec<u8>, mut value: i32) -> Result<(), PacketWriteError> {
        loop {
            let mut temp = (value & 0b0111_1111) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0b1000_0000;
            }
            buffer.write_all(&[temp])?;
            if value == 0 {
                break;
            }
        }
        Ok(())
    }

    pub fn read_uuid(buffer: &mut &[u8]) -> Result<Uuid, PacketReadError> {
        let mut uuid_buf = [0u8; 16];
        buffer.read_exact(&mut uuid_buf)?;
        Ok(Uuid::from_bytes(uuid_buf))
    }

    pub fn read_unsigned_short(buffer: &mut &[u8]) -> Result<u16, PacketReadError> {
        let mut short_buf = [0u8; 2];
        buffer.read_exact(&mut short_buf)?;
        Ok(u16::from_be_bytes(short_buf))
    }

    pub fn write_unsigned_short(buffer: &mut Vec<u8>, value: u16) -> Result<(), PacketWriteError> {
        buffer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    pub fn read_long(buffer: &mut &[u8]) -> Result<i64, PacketReadError> {
        let mut long_buf = [0u8; 8];
        buffer.read_exact(&mut long_buf)?;
        Ok(i64::from_be_bytes(long_buf))
    }

    pub fn write_long(buffer: &mut Vec<u8>, value: i64) -> Result<(), PacketWriteError> {
        buffer.write_all(&value.to_be_bytes())?;
        Ok(())
    }

    pub fn write_uuid(buffer: &mut Vec<u8>, value: Uuid) -> Result<(), PacketWriteError> {
        buffer.write_all(value.as_bytes())?;
        Ok(())
    }
}

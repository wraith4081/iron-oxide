use std::io::Read;
use uuid::Uuid;
use crate::error::{Error, Result};

pub fn read_varint(buffer: &mut &[u8]) -> Result<i32> {
    let mut num_read = 0;
    let mut result = 0;
    let mut read;
    loop {
        if num_read >= 5 {
            return Err(Error::InvalidVarInt);
        }
        let mut temp = [0];
        buffer.read_exact(&mut temp)?;
        read = temp[0];
        let value = (read & 0b01111111) as i32;
        result |= value << (7 * num_read);
        num_read += 1;
        if (read & 0b10000000) == 0 {
            break;
        }
    }
    Ok(result)
}

pub fn write_varint(buffer: &mut Vec<u8>, mut value: i32) -> Result<()> {
    loop {
        let mut temp = (value & 0b01111111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b10000000;
        }
        buffer.push(temp);
        if value == 0 {
            break;
        }
    }
    Ok(())
}

pub fn read_string(buffer: &mut &[u8]) -> Result<String> {
    let len = read_varint(buffer)? as usize;
    if len > 32767 {
        return Err(Error::InvalidString);
    }
    let mut str_buf = vec![0; len];
    buffer.read_exact(&mut str_buf)?;
    String::from_utf8(str_buf).map_err(|_| Error::InvalidString)
}

pub fn write_string(buffer: &mut Vec<u8>, value: &str) -> Result<()> {
    write_varint(buffer, value.len() as i32)?;
    buffer.extend_from_slice(value.as_bytes());
    Ok(())
}

pub fn read_unsigned_short(buffer: &mut &[u8]) -> Result<u16> {
    let mut buf = [0; 2];
    buffer.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

pub fn write_unsigned_short(buffer: &mut Vec<u8>, value: u16) -> Result<()> {
    buffer.extend_from_slice(&value.to_be_bytes());
    Ok(())
}

pub fn read_long(buffer: &mut &[u8]) -> Result<i64> {
    let mut buf = [0; 8];
    buffer.read_exact(&mut buf)?;
    Ok(i64::from_be_bytes(buf))
}

pub fn write_long(buffer: &mut Vec<u8>, value: i64) -> Result<()> {
    buffer.extend_from_slice(&value.to_be_bytes());
    Ok(())
}

pub fn read_uuid(buffer: &mut &[u8]) -> Result<Uuid> {
    let mut buf = [0; 16];
    buffer.read_exact(&mut buf)?;
    Ok(Uuid::from_bytes(buf))
}

pub fn write_uuid(buffer: &mut Vec<u8>, value: Uuid) -> Result<()> {
    buffer.extend_from_slice(value.as_bytes());
    Ok(())
}

pub fn read_bytes<'a>(buffer: &mut &'a [u8], len: usize) -> Result<&'a [u8]> {
    if buffer.len() < len {
        return Err(Error::UnexpectedEof);
    }
    let (bytes, rest) = buffer.split_at(len);
    *buffer = rest;
    Ok(bytes)
}

pub fn write_varint_prefixed_array<T, F>(buffer: &mut Vec<u8>, array: &[T], mut writer: F) -> Result<()>
where
    F: FnMut(&mut Vec<u8>, &T) -> Result<()>,
{
    write_varint(buffer, array.len() as i32)?;
    for item in array {
        writer(buffer, item)?;
    }
    Ok(())
}
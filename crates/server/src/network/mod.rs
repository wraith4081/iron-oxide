use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn read_varint(stream: &mut TcpStream) -> io::Result<i32> {
    let mut num_read = 0;
    let mut result = 0;
    let mut read_byte;
    loop {
        read_byte = stream.read_u8().await?;
        let value = (read_byte & 0b0111_1111) as i32;
        result |= value << (7 * num_read);
        num_read += 1;
        if num_read > 5 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "VarInt is too big",
            ));
        }
        if (read_byte & 0b1000_0000) == 0 {
            break;
        }
    }
    Ok(result)
}

pub async fn write_varint(stream: &mut TcpStream, mut value: i32) -> io::Result<()> {
    loop {
        let mut temp = (value & 0b0111_1111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b1000_0000;
        }
        stream.write_u8(temp).await?;
        if value == 0 {
            break;
        }
    }
    Ok(())
}

pub fn write_varint_to_vec(vec: &mut Vec<u8>, mut value: i32) -> io::Result<()> {
    loop {
        let mut temp = (value & 0b0111_1111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b1000_0000;
        }
        vec.push(temp);
        if value == 0 {
            break;
        }
    }
    Ok(())
}

pub fn write_string_to_vec(vec: &mut Vec<u8>, value: &str) -> io::Result<()> {
    let bytes = value.as_bytes();
    write_varint_to_vec(vec, bytes.len() as i32)?;
    vec.extend(bytes);
    Ok(())
}

use std::io;
use std::sync::Arc;
use async_trait::async_trait;
use bytes::{Buf, BytesMut};
use iron_oxide_protocol::error::{Error, Result};
use iron_oxide_protocol::packet::Packet;
use iron_oxide_protocol::packet::raw_data::{read_varint, write_varint};
use iron_oxide_protocol::stream::ConnectionIO;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::config::Config;

pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
    pub state: ConnectionState,
    pub config: Arc<Config>,
    pub protocol_version: i32,
}

impl Connection {
    pub fn new(stream: TcpStream, config: Arc<Config>) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(4096),
            state: ConnectionState::Handshaking,
            config,
            protocol_version: 0,
        }
    }

    pub async fn read_packet<T: Packet + Send>(&mut self) -> Result<Option<T>> {
        loop {
            if let Some(packet) = self.parse_packet::<T>()? {
                return Ok(Some(packet));
            }

            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err(Error::Io(io::Error::new(
                        io::ErrorKind::ConnectionAborted,
                        "Connection closed by peer",
                    )))
                };
            }
        }
    }

    pub async fn peek_packet(&mut self) -> Result<&[u8]> {
        loop {
            if !self.buffer.is_empty() {
                return Ok(&self.buffer[..]);
            }

            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                return if self.buffer.is_empty() {
                    Ok(&[])
                } else {
                    Err(Error::Io(io::Error::new(
                        io::ErrorKind::ConnectionAborted,
                        "Connection closed by peer",
                    )))
                };
            }
        }
    }

    pub async fn read_packet_raw(&mut self) -> Result<Option<BytesMut>> {
        loop {
            if let Some(packet) = self.parse_packet_raw()? {
                return Ok(Some(packet));
            }

            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err(Error::Io(io::Error::new(
                        io::ErrorKind::ConnectionAborted,
                        "Connection closed by peer",
                    )))
                };
            }
        }
    }

    fn parse_packet<T: Packet>(&mut self) -> Result<Option<T>> {
        let mut buf = &self.buffer[..];
        let initial_len = buf.len();

        if initial_len == 0 {
            return Ok(None);
        }

        let packet_len = match read_varint(&mut buf) {
            Ok(len) => len,
            Err(Error::Io(e)) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        };

        if buf.len() < packet_len as usize {
            return Ok(None);
        }

        let packet_len_len = initial_len - buf.len();
        let total_packet_len = packet_len_len + packet_len as usize;

        let packet_data = &self.buffer[packet_len_len..total_packet_len];
        let mut packet_data_slice = &packet_data[..];
        let _packet_id = read_varint(&mut packet_data_slice)?;

        let packet = T::read(&mut packet_data_slice)?;
        self.buffer.advance(total_packet_len);

        Ok(Some(packet))
    }

    fn parse_packet_raw(&mut self) -> Result<Option<BytesMut>> {
        let mut buf = &self.buffer[..];
        let initial_len = buf.len();

        if initial_len == 0 {
            return Ok(None);
        }

        let packet_len = match read_varint(&mut buf) {
            Ok(len) => len,
            Err(Error::Io(e)) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        };

        if buf.len() < packet_len as usize {
            return Ok(None);
        }

        let packet_len_len = initial_len - buf.len();
        let total_packet_len = packet_len_len + packet_len as usize;

        let packet_data = self.buffer.split_to(total_packet_len);
        Ok(Some(packet_data))
    }

    pub async fn write_packet<T: Packet + Send>(&mut self, packet: T) -> Result<()> {
        let mut buf = Vec::new();
        packet.write(&mut buf)?;

        let mut final_buf = Vec::new();
        write_varint(&mut final_buf, buf.len() as i32)?;
        final_buf.extend_from_slice(&buf);

        self.stream.write_all(&final_buf).await?;
        Ok(())
    }
}

#[async_trait]
impl ConnectionIO for Connection {
    async fn read_packet_io<T: Packet + Send>(&mut self) -> Result<Option<T>> {
        self.read_packet().await
    }

    async fn write_packet_io<T: Packet + Send>(&mut self, packet: T) -> Result<()> {
        self.write_packet(packet).await
    }
}
use anyhow::Result;
use async_trait::async_trait;
use iron_oxide_protocol::packet::{Packet, PacketReadError, PacketWriteError};

#[async_trait]
pub trait ConnectionIO {
    async fn read_packet_io<T: Packet + Send>(&mut self) -> Result<Option<T>, PacketReadError>;
    async fn write_packet_io<T: Packet + Send>(&mut self, packet: T) -> Result<(), PacketWriteError>;
}

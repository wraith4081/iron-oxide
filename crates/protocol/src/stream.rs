use async_trait::async_trait;
use crate::error::Result;
use crate::packet::Packet;

#[async_trait]
pub trait ConnectionIO {
    async fn read_packet_io<T: Packet + Send>(&mut self) -> Result<Option<T>>;
    async fn write_packet_io<T: Packet + Send>(&mut self, packet: T) -> Result<()>;
}
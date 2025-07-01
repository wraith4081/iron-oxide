pub mod raw_data;
pub mod data;
pub mod types;

use crate::error::Result;

pub trait Packet: Sized {
    fn read(buffer: &mut &[u8]) -> Result<Self> where Self: Sized;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<()>;
}

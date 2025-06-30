pub mod data;

use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketReadError {
    #[error("IO error: {0}")]
    IO(#[from] io::Error),
    #[error("Invalid VarInt")]
    InvalidVarInt,
    #[error("Invalid String")]
    InvalidString,
    #[error("Invalid UUID")]
    InvalidUuid,
    #[error("Unexpected EOF")]
    UnexpectedEof,
}

#[derive(Debug, Error)]
pub enum PacketWriteError {
    #[error("IO error: {0}")]
    IO(#[from] io::Error),
}

pub trait Packet: Sized {
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketReadError> where Self: Sized;
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError>;
}
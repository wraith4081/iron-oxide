use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid VarInt")]
    InvalidVarInt,

    #[error("Invalid String")]
    InvalidString,

    #[error("Invalid UUID")]
    InvalidUuid,

    #[error("Unexpected EOF")]
    UnexpectedEof,

    #[error("Packet too large")]
    PacketTooLarge,

    #[error("Invalid packet ID: {0}")]
    InvalidPacketId(i32),

    #[error("Packet serialization error: {0}")]
    PacketSerialization(String),

    #[error("Packet deserialization error: {0}")]
    PacketDeserialization(String),

    #[error("Incomplete packet")]
    IncompletePacket,

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(i32),
}

#[derive(Debug, Error)]
pub enum VersionError {
    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(i32),
}

impl From<VersionError> for Error {
    fn from(e: VersionError) -> Self {
        match e {
            VersionError::UnsupportedVersion(v) => Error::UnsupportedVersion(v),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

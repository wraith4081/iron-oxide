use iron_oxide_protocol::packet::{Packet, PacketReadError, PacketWriteError};
use iron_oxide_protocol::packet::data::{read_long, write_long, write_string, write_varint};
use serde::{Deserialize, Serialize};

pub struct StatusRequest;

impl Packet for StatusRequest {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        Ok(Self)
    }

    fn write(&self, _: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        unimplemented!()
    }
}

pub struct StatusResponse {
    pub response: String,
}

impl Packet for StatusResponse {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        unimplemented!()
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        write_varint(buffer, 0x00)?;
        write_string(buffer, &self.response)?;
        Ok(())
    }
}

pub struct PingRequest {
    pub payload: i64,
}

impl Packet for PingRequest {
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketReadError> {
        let payload = read_long(buffer)?;
        Ok(Self { payload })
    }

    fn write(&self, _: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        unimplemented!()
    }
}

pub struct PongResponse {
    pub payload: i64,
}

impl Packet for PongResponse {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        unimplemented!()
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        write_varint(buffer, 0x01)?;
        write_long(buffer, self.payload)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub version: Version,
    pub players: Players,
    pub description: Description,
    #[serde(rename = "favicon")]
    pub favicon: Option<String>,
}

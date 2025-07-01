use uuid::Uuid;
use crate::error::{Error, Result};
use crate::packet::data::{read_string, read_uuid, write_string, write_uuid};
use crate::packet::Packet;

pub struct LoginStart {
    pub name: String,
    pub uuid: Uuid,
}

impl Packet for LoginStart {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        let name = read_string(buffer)?;
        let uuid = read_uuid(buffer)?;
        Ok(Self { name, uuid })
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        write_string(buffer, &self.name)?;
        write_uuid(buffer, self.uuid)?;
        Ok(())
    }
}

pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
}

impl Packet for LoginSuccess {
    fn read(_: &mut &[u8]) -> Result<Self> {
        Err(Error::Protocol("Client cannot send LoginSuccess packet".to_string()))
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        write_uuid(buffer, self.uuid)?;
        write_string(buffer, &self.username)?;
        // TODO: properties
        buffer.push(0);
        Ok(())
    }
}
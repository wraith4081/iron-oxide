use iron_oxide_protocol::packet::{Packet, PacketReadError, PacketWriteError};
use iron_oxide_protocol::packet::data::{read_string, read_uuid, write_string, write_uuid, read_varint, write_varint};
use uuid::Uuid;

pub struct LoginStart {
    pub name: String,
    pub uuid: Uuid,
}

impl Packet for LoginStart {
    fn read(buffer: &mut &[u8]) -> Result<Self, PacketReadError> {
        let name = read_string(buffer)?;
        let uuid = read_uuid(buffer)?;
        Ok(Self { name, uuid })
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        write_string(buffer, &self.name)?;
        write_uuid(buffer, self.uuid)?;
        Ok(())
    }
}

pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<Property>,
    pub enforce_secure_chat: bool,
}

impl Packet for LoginSuccess {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        unimplemented!()
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        write_varint(buffer, 0x02)?;
        write_uuid(buffer, self.uuid)?;
        write_string(buffer, &self.username)?;
        write_varint(buffer, self.properties.len() as i32)?;
        for property in &self.properties {
            property.write(buffer)?;
        }
        buffer.push(if self.enforce_secure_chat { 1 } else { 0 });
        Ok(())
    }
}

pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl Property {
    fn write(&self, buffer: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        write_string(buffer, &self.name)?;
        write_string(buffer, &self.value)?;
        match &self.signature {
            Some(signature) => {
                buffer.push(1);
                write_string(buffer, signature)?;
            }
            None => {
                buffer.push(0);
            }
        }
        Ok(())
    }
}

pub struct LoginAcknowledged;

impl Packet for LoginAcknowledged {
    fn read(_: &mut &[u8]) -> Result<Self, PacketReadError> {
        Ok(Self)
    }

    fn write(&self, _: &mut Vec<u8>) -> Result<(), PacketWriteError> {
        unimplemented!()
    }
}

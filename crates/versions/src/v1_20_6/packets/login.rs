use iron_oxide_protocol::error::Result;
use iron_oxide_protocol::packet;
use uuid::Uuid;

packet! {
    #[derive(Debug)]
    pub struct LoginStart(0x00) {
        name: String,
        uuid: Uuid,
    }
}

fn read_properties(buffer: &mut &[u8]) -> Result<Vec<Property>> {
    let len: i32 = packet::data::PacketData::read(buffer)?;
    let mut properties = Vec::with_capacity(len as usize);
    for _ in 0..len {
        properties.push(Property::read(buffer)?);
    }
    Ok(properties)
}

fn write_properties(properties: &Vec<Property>, buffer: &mut Vec<u8>) -> Result<()> {
    packet::data::PacketData::write(&(properties.len() as i32), buffer)?;
    for property in properties {
        property.write(buffer)?;
    }
    Ok(())
}

packet! {
    #[derive(Debug)]
    pub struct LoginSuccess(0x02) {
        uuid: Uuid,
        username: String,
        properties: Vec<Property> = (read_properties, write_properties),
        enforce_secure_chat: bool,
    }
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl Property {
    fn read(buffer: &mut &[u8]) -> Result<Self> {
        let name = packet::data::PacketData::read(buffer)?;
        let value = packet::data::PacketData::read(buffer)?;
        let has_signature: bool = packet::data::PacketData::read(buffer)?;
        let signature = if has_signature {
            Some(packet::data::PacketData::read(buffer)?)
        } else {
            None
        };
        Ok(Self {
            name,
            value,
            signature,
        })
    }

    fn write(&self, buffer: &mut Vec<u8>) -> Result<()> {
        packet::data::PacketData::write(&self.name, buffer)?;
        packet::data::PacketData::write(&self.value, buffer)?;
        match &self.signature {
            Some(signature) => {
                packet::data::PacketData::write(&true, buffer)?;
                packet::data::PacketData::write(signature, buffer)?;
            }
            None => {
                packet::data::PacketData::write(&false, buffer)?;
            }
        }
        Ok(())
    }
}

packet! {
    #[derive(Debug)]
    pub struct LoginAcknowledged(0x03) {}
}


#[macro_export]
macro_rules! packet {
    (
        $(#[$outer:meta])*
        pub struct $name:ident($id:expr) {
            $($field:ident: $type:ty $(= ($read:expr, $write:expr))?),* $(,)?
        }
    ) => {
        $(#[$outer])*
        pub struct $name {
            $(pub $field: $type),*
        }

        impl iron_oxide_protocol::packet::Packet for $name {
            fn read(buffer: &mut &[u8]) -> Result<Self, iron_oxide_protocol::packet::PacketReadError> {
                $(
                    let $field = packet!(@read_field buffer, $type, $($read)?);
                )*
                Ok(Self {
                    $($field),*
                })
            }

            fn write(&self, buffer: &mut Vec<u8>) -> Result<(), iron_oxide_protocol::packet::PacketWriteError> {
                iron_oxide_protocol::packet::raw_data::write_varint(buffer, $id)?;
                $(
                    packet!(@write_field buffer, &self.$field, $($write)?);
                )*
                Ok(())
            }
        }
    };

    (@read_field $buffer:expr, $type:ty, $read:expr) => {
        $read($buffer)?
    };

    (@read_field $buffer:expr, $type:ty,) => {
        iron_oxide_protocol::packet::data::PacketData::read($buffer)?
    };

    (@write_field $buffer:expr, $field:expr, $write:expr) => {
        $write($field, $buffer)?
    };

    (@write_field $buffer:expr, $field:expr,) => {
        iron_oxide_protocol::packet::data::PacketData::write($field, $buffer)?
    };
}


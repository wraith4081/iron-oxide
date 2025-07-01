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

        impl $crate::packet::Packet for $name {
            fn read(buffer: &mut &[u8]) -> $crate::error::Result<Self> {
                $(
                    let $field = packet!(@read_field buffer, $type, $($read)?);
                )*
                Ok(Self {
                    $($field),*
                })
            }

            fn write(&self, buffer: &mut Vec<u8>) -> $crate::error::Result<()> {
                $crate::packet::raw_data::write_varint(buffer, $id)?;
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
        $crate::packet::data::PacketData::read($buffer)?
    };

    (@write_field $buffer:expr, $field:expr, $write:expr) => {
        $write($field, $buffer)?
    };

    (@write_field $buffer:expr, $field:expr,) => {
        $crate::packet::data::PacketData::write($field, $buffer)?
    };
}
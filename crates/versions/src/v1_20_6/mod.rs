use crate::Version;

pub mod packets;
pub mod handlers;

pub const V1_20_6: VersionImpl = VersionImpl;

pub struct VersionImpl;

impl Version for VersionImpl {
    fn protocol_version(&self) -> i32 {
        766
    }
}

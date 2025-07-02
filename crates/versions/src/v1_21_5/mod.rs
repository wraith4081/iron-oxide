use crate::Version;

pub mod packets;
pub mod handlers;

pub const V1_21_5: VersionImpl = VersionImpl;

pub struct VersionImpl;

impl Version for VersionImpl {
    fn protocol_version(&self) -> i32 {
        770
    }
}

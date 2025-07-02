use iron_oxide_protocol::error::VersionError;

pub mod v1_21_5;

pub trait Version {
    fn protocol_version(&self) -> i32;
}

pub struct VersionManager;

impl VersionManager {
    pub fn get_version(protocol_version: i32) -> Result<&'static dyn Version, VersionError> {
        match protocol_version {
            770 => Ok(&v1_21_5::VersionImpl),
            _ => Err(VersionError::UnsupportedVersion(protocol_version)),
        }
    }
}
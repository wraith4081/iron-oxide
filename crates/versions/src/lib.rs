use thiserror::Error;

pub mod v1_20_6;
pub mod stream;

#[derive(Debug, Error)]
pub enum VersionError {
    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(i32),
}

pub trait Version {
    fn protocol_version(&self) -> i32;
}

pub struct VersionManager;

impl VersionManager {
    pub fn get_version(protocol_version: i32) -> Result<&'static dyn Version, VersionError> {
        match protocol_version {
            766 => Ok(&v1_20_6::VersionImpl),
            _ => Err(VersionError::UnsupportedVersion(protocol_version)),
        }
    }
}

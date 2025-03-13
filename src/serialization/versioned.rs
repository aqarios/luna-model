use prost::{DecodeError, Message};

#[repr(u32)]
pub enum Version {
    V0 = 0,
}

impl Version {
    pub fn latest() -> Self {
        Version::V0
    }
}

#[derive(Message)]
pub struct Versioned {
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(bytes, tag = "2")]
    pub data: Vec<u8>,
}

impl Versioned {
    fn new(version: Version, data: Vec<u8>) -> Self {
        Self {
            version: version as u32,
            data,
        }
    }
}

pub fn versionize(data: Vec<u8>, version: Version) -> Vec<u8> {
    Versioned::new(version, data).encode_to_vec()
}

pub fn unversionize(data: &[u8]) -> Result<Versioned, DecodeError> {
    Versioned::decode(data)
}

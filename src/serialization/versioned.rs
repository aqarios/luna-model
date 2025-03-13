use prost::Message;

#[repr(u32)]
pub enum Version {
    V0 = 0,
}

impl Version {
    pub fn latest() -> Self {
        Version::V0
    }

    pub fn from(u: u32) -> Self {
        match u {
            0 => Version::V0,
            _ => panic!("unkown version"),
        }
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
    pub fn new(version: Version, data: Vec<u8>) -> Self {
        Self {
            version: version as u32,
            data,
        }
    }
}

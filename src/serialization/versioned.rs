use prost::Message;

pub static VERSION_0_0_1: &str = "v0.0.1";

// #[repr(i32)]
// pub enum Version {
//     V1 = 0,
// }
//
// impl Display for Version {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Version::V1 => write!(f, "v0.0.1"),
//         }
//     }
// }

#[derive(Message)]
pub struct Versioned {
    #[prost(string, tag = "1")]
    pub version: String,
    #[prost(bytes, tag = "2")]
    pub data: Vec<u8>,
}

impl Versioned {
    pub fn new<T: Message>(value: T) -> Self {
        Self {
            version: VERSION_0_0_1.to_string(),
            data: value.encode_to_vec(),
        }
    }

    pub fn encoded<T: Message>(value: T) -> Vec<u8> {
        let versioned = Versioned::new(value);
        versioned.encode_to_vec()
    }
}

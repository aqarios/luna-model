use prost::Message;

// pub static DEFAULT_COMPRESSION_LEVEL: i32 = 3;

#[derive(Clone, PartialEq, Message)]
pub struct Compressed {
    #[prost(bool, tag = "1")]
    pub compressed: bool,
    #[prost(bytes, tag = "2")]
    pub data: Vec<u8>,
}

impl Compressed {
    pub fn new(compressed: bool, data: Vec<u8>) -> Self {
        Self { compressed, data }
    }
}

// pub fn compress(
//     bytes: Vec<u8>,
//     use_compression: bool,
//     level: Option<i32>,
// ) -> Result<Vec<u8>, io::Error> {
//     match use_compression {
//         true => {
//             let compressed =
//                 zstd::encode_all(bytes.as_slice(), level.unwrap_or(DEFAULT_COMPRESSION_LEVEL))?;
//             Ok(Compressed::new(true, compressed).encode_to_vec())
//         }
//         false => Ok(Compressed::new(false, bytes).encode_to_vec()),
//     }
// }
//
// pub fn decompress(bytes: &[u8]) -> Result<Vec<u8>, DecodeError> {
//     let c = Compressed::decode(bytes)?;
//     match c.compressed {
//         false => Ok(c.data),
//         true => zstd::decode_all(c.data.as_slice()).map_err(|e| DecodeError::new(e.to_string())),
//     }
// }

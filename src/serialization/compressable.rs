use std::io;

use prost::Message;

use super::{
    utils::{Slicable, Vectorizeable},
    versionizable::Finalize,
};

pub static DEFAULT_COMPRESSION_LEVEL: i32 = 3;

#[derive(Clone, PartialEq, Message)]
pub struct SerCompressed {
    #[prost(bool, tag = "1")]
    pub compressed: bool,
    #[prost(bytes, tag = "2")]
    pub data: Vec<u8>,
}

impl SerCompressed {
    fn new(compressed: bool, data: Vec<u8>) -> Self {
        Self { compressed, data }
    }
}

pub trait Compressable
where
    Self: Sized + Slicable + Vectorizeable,
{
    fn compress(&self, level: Option<i32>) -> Result<Vec<u8>, io::Error> {
        zstd::encode_all(self.as_slice(), level.unwrap_or(DEFAULT_COMPRESSION_LEVEL))
    }

    /// Maybe compress itself. In contrast to `compress` this
    /// function also takes a bool. This method should be
    /// used to ease information flow when compression might
    /// not be desired in all cases.
    fn maybe_compress(
        self,
        do_compression: bool,
        level: Option<i32>,
    ) -> Result<Vec<u8>, io::Error> {
        match do_compression {
            true => Ok(SerCompressed::new(true, self.compress(level)?).encode_to_vec()),
            false => Ok(SerCompressed::new(false, self.to_vec()).encode_to_vec()),
        }
    }
}

pub trait Decompressable<D = Self>
where
    Self: Sized + Slicable,
    D: From<Self>,
    Self: Finalize<Vec<u8>>,
{
    fn decompress(self) -> Result<D, io::Error> {
        let result = SerCompressed::decode(self.as_slice());
        match result {
            Ok(compressed) => match compressed.compressed {
                true => Ok(self
                    .finalize(zstd::decode_all(compressed.data.as_slice())?)
                    .into()),
                false => Ok(self.finalize(compressed.data).into()),
            },
            Err(_) => Ok(self.into()),
        }
    }
}

impl Slicable for Vec<u8> {
    fn as_slice(&self) -> &[u8] {
        self.as_slice()
    }
}
impl Compressable for Vec<u8> {}

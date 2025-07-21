use std::io;

use prost::Message;

use crate::errors::CompressionErr;

use super::{
    utils::{Slicable, Vectorizable},
    versionizable::Versioned,
};

/// The default compression level used for the `zstd` compression algorithm.
pub static DEFAULT_COMPRESSION_LEVEL: i32 = 3;

/// A serializable struct defining the data layout for the protocol buffer based
/// encoding and decoding. Used internally to implement the encoding and decoding
/// capabilities using protocol buffers. Defines if the `data` is compressed required
/// in the decoding step.
#[derive(Clone, PartialEq, Message)]
struct SerCompressed {
    #[prost(bool, tag = "1")]
    pub compressed: bool,
    #[prost(bytes, tag = "2")]
    pub data: Vec<u8>,
}

impl SerCompressed {
    /// Create a new instance of a serializable compression representation.
    fn new(compressed: bool, data: Vec<u8>) -> Self {
        Self { compressed, data }
    }
}

/// Defines common methods for interacting with compressable types.
pub trait Compressable
where
    Self: Sized + Slicable + Vectorizable,
{
    /// Compress self to a bytes vector using the specified lavel if not None,
    /// otherwise the DEFAULT_COMPRESSION_LEVEL is used.
    fn compress(&self, level: Option<i32>) -> Result<Vec<u8>, io::Error> {
        zstd::encode_all(self.as_slice(), level.unwrap_or(DEFAULT_COMPRESSION_LEVEL))
    }

    /// Maybe compress self to a bytes vector using the specified level.
    /// In contrast to the `compress` method, this method also takes a bool as an input
    /// paramater.
    fn maybe_compress(
        self,
        do_compression: Option<bool>,
        level: Option<i32>,
    ) -> Result<Vec<u8>, CompressionErr> {
        match do_compression {
            Some(true) => Ok(SerCompressed::new(true, self.compress(level)?).encode_to_vec()),
            Some(false) | None => Ok(SerCompressed::new(false, self.to_vec()).encode_to_vec()),
        }
    }
}

/// Defines common methods for interacting with decompressable types.
pub trait Decompressable<D = Self>
where
    Self: Sized + Slicable,
    D: From<Self>,
    Self: Finalize<Vec<u8>>,
{
    /// Decompress self to an instance of type D.
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

/// This is a utility trait required for recovering self based on some input data `D`.
pub trait Finalize<D> {
    /// Based on the provided input of type `D` update and return the instance of `Self`.
    fn finalize(self, input: D) -> Self;
}

/// Implementation of Finalize for a Versioned bytes vector to populate the versioned
/// instances with the bytes array.
impl Finalize<Vec<u8>> for Versioned<Vec<u8>> {
    /// Takes a bytes vector and populates the data of the versioned instance `self`.
    fn finalize(mut self, input: Vec<u8>) -> Self {
        self.data = input;
        self
    }
}

/// Enables the compression capabilities for a bytes vector.
impl Compressable for Vec<u8> {}

/// Enables the decompression capabilities for a versioned bytes vector.
impl Decompressable for Versioned<Vec<u8>> {}

impl From<io::Error> for CompressionErr {
    fn from(value: io::Error) -> Self {
        CompressionErr(value.to_string())
    }
}

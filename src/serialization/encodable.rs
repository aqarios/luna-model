use std::io;

use super::{
    compression::compress,
    versioned::{versionize, Version},
};

pub trait BytesEncodable {
    fn encode_to_bytes(&self) -> Vec<u8>;
}

pub trait BytesDecodeable {
    fn extract(&self) -> Vec<u8>;
}

pub trait Creatable<T>
where
    Self: BytesEncodable,
{
    fn new(value: &T) -> Self;
}

pub trait Encodable<S>
where
    Self: Sized,
    S: Creatable<Self>,
{
    fn encode(&self, use_compression: bool, level: Option<i32>) -> Result<Vec<u8>, io::Error> {
        compress(S::new(&self).encode_to_bytes(), use_compression, level)
    }

    fn versionized(
        &self,
        use_compression: bool,
        level: Option<i32>,
        version: Option<Version>,
    ) -> Result<Vec<u8>, io::Error> {
        Ok(versionize(
            self.encode(use_compression, level)?,
            version.unwrap_or(Version::latest()),
        ))
    }
}

use prost::Message;

use super::{
    utils::{Slicable, Vectorizeable},
    versioned::{Version, Versioned as SerVersioned},
    Decompressable,
};

pub struct Versioned<B> {
    pub version: Option<Version>,
    pub data: B,
}

impl<B> Versioned<B> {
    fn unknown(data: B) -> Self {
        Self {
            version: None,
            data,
        }
    }

    fn new(version: Version, data: B) -> Self {
        Self {
            version: Some(version),
            data,
        }
    }
}

pub trait Versionizable<B = Self>
where
    Self: Sized + Vectorizeable,
{
    fn versionize(self) -> Vec<u8> {
        SerVersioned::new(Version::latest(), self.to_vec()).encode_to_vec()
    }
}

pub trait Unversionizable
where
    Self: Sized + Slicable,
{
    fn unversionize(&self) -> Versioned<Vec<u8>> {
        let result = SerVersioned::decode(self.as_slice());
        match result {
            Ok(versioned) => {
                Versioned::new(Version::from(versioned.version), versioned.data.into())
            }
            Err(_) => {
                // Unversioned data...
                Versioned::unknown(self.as_slice().to_vec())
            }
        }
    }
}

impl Vectorizeable for Vec<u8> {
    fn to_vec(self) -> Vec<u8> {
        self
    }
}

impl Slicable for &[u8] {
    fn as_slice(&self) -> &[u8] {
        self
    }
}

impl Versionizable for Vec<u8> {}
impl Unversionizable for &[u8] {}

impl Slicable for Versioned<Vec<u8>> {
    fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }
}

pub trait Finalize<D> {
    fn finalize(self, input: D) -> Self;
}

impl Finalize<Vec<u8>> for Versioned<Vec<u8>> {
    fn finalize(mut self, input: Vec<u8>) -> Self {
        self.data = input;
        self
    }
}

impl Decompressable for Versioned<Vec<u8>> {}

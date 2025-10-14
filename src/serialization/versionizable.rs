use prost::Message;

use super::utils::{Slicable, Vectorizable};

/// A unique version identifier used to specify which serialization was used.
#[repr(u32)]
pub enum Version {
    V0 = 0,
    V1 = 1,
}

/// Utility methods for working on the version enum.
impl Version {
    /// Helper function to recover the version as an Enum based on a u32.
    pub fn from(u: u32) -> Self {
        match u {
            0 => Version::V0,
            1 => Version::V1,
            _ => panic!("unkown version"),
        }
    }
}

/// A serializable version structure defining the data layout for protocol buffer
/// based encoding and decoding. Used internally to implement the encoding and decoding
/// capabilities using protocol buffers.
#[derive(Message)]
struct SerVersioned {
    /// The version of the serialization which was used to encode the `data`.
    #[prost(uint32, tag = "1")]
    version: u32,
    /// A serialized object expressed as bytes.
    #[prost(bytes, tag = "2")]
    data: Vec<u8>,
}

impl SerVersioned {
    /// Create a new serializable versioned stucture.
    fn new(version: Version, data: Vec<u8>) -> Self {
        Self {
            version: version as u32,
            data,
        }
    }
}

/// An abstraction of the `SerVersioned` used as a clearly defined input and output
/// independent of the actual implementation used for serialization/encoding.
/// Uses the generic `B` representing an arbitrary data type for expressing the data.
pub struct Versioned<B> {
    /// The specific version of encoding used for data. The version is defined as an
    /// optional to be able to handle unversionized data using the same data flow as
    /// versionized data. How unversionized data is handled is a task for the user of
    /// a versioned structure.
    pub version: Option<Version>,
    /// The data that is maybe versionized.
    pub data: B,
}

/// Utility methods used for creation of a new `Versioned` instance.
impl<B> Versioned<B> {
    /// Create an instance for an unknown version.
    /// This method is needed to be able to express unversionized data. While ensuring
    /// correct data alignment for methods that expect versioned data.
    fn unknown(data: B) -> Self {
        Self {
            version: None,
            data,
        }
    }

    /// Create an instance for an specific version.
    fn new(version: Version, data: B) -> Self {
        Self {
            version: Some(version),
            data,
        }
    }
}

/// This trait defines the required methods for an object to be versionizable.
pub trait Versionizable<B = Self>
where
    Self: Sized + Vectorizable,
{
    /// Package the data as a serializable versionized packet. This verison is always
    /// set to latest available version. The user must ensure that it's implementation
    /// ensures that the underlying serialization/encoding of the data uses the latest
    /// version as well.
    ///
    /// If your requirement is to versionize data that is not always
    /// the latest version, please create a new Issue [here](https://github.com/aqarios/aq-models-rs/issues)
    /// explaining your use case and why you require a more flexible versionizing approach.
    fn versionize(self, version: Version) -> Vec<u8> {
        SerVersioned::new(version, self.to_vec()).encode_to_vec()
    }
}

/// This trait defines the required methods for an object to be unversionizable, i.e.,
/// extract the version and the
pub trait Unversionizable
where
    Self: Sized + Slicable,
{
    /// Extract self to a `Versioned` struct with the date being expressed as a vector
    /// of bytes.
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

/// For an object to be Versionizable it has to be vectorizable, i.e., it can be expressed
/// as a bytes array. This is automatically true for a Vec<u8> as it directly represents a
/// bytes array. However, we require it to implement the method expected by the Versionizable
/// trait.
impl Vectorizable for Vec<u8> {
    /// Returns self since a Vec<u8> already is a bytes array.
    #[inline]
    fn to_vec(self) -> Vec<u8> {
        self
    }
}

/// For an object to be Unversionizable it has to be Slicable, i.e., it can be expressed
/// as a slice of bytes. This is automatically true for a &[u8] as it directly represents a
/// slice of bytes. However, we require it to implement the method expected by the
/// Unversionizable trait.
impl Slicable for &[u8] {
    /// Returns self since a &[u8] already is a slice of bytes.
    #[inline]
    fn as_slice(&self) -> &[u8] {
        self
    }
}

/// Implementation of the Versionizable trait for the `Vec<u8>` data type.
/// Since the Versionizable trait implements it's methods by default and `Vec<u8>` fullfills
/// the Vectorizable trait we are not required to adjust the default implementation.
impl Versionizable for Vec<u8> {}

/// Implementation of the Unversionizable trait for the &[u8] data type.
/// Since the Unversionizable trait implements it's methods by default and &[u8] fullfills
/// the Slicable trait we are not required to adjust the default implementation.
impl Unversionizable for &[u8] {}

/// For an object to be Unversionizable it has to be Slicable, i.e., it can be expressed
/// as a slice of bytes. This is automatically true for a &[u8] as it directly represents a
/// slice of bytes. However, for a Versioned<Vec<u8>> we need to return it's data as a slice.
impl Slicable for Versioned<Vec<u8>> {
    /// Return the versioned data as a slice.
    fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }
}

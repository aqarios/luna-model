use super::utils::Slicable;

/// An erorr returned on issues in the decoding/deserialization of data.
#[derive(Debug, Clone)]
pub struct DecodeError {
    /// The specific message describing the decoding error.
    msg: String,
}

impl DecodeError {
    fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl std::error::Error for DecodeError {}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "decoding failed: {}", self.msg)
    }
}

impl std::convert::From<prost::DecodeError> for DecodeError {
    fn from(err: prost::DecodeError) -> DecodeError {
        DecodeError::new(err.to_string())
    }
}

/// Defines the common interface to create a bytes encoded instance of self.
pub trait BytesEncodable {
    fn encode_to_bytes(&self) -> Vec<u8>;
}

/// Defines the common interface to decode to S based on it's representation as bytes.
pub trait BytesDecodable<S, P = ()> {
    fn decode_from_bytes(bytes: &[u8], payload: P) -> Result<S, DecodeError>;
}

/// Defines the common interface required by the Encodable trait for an object to
/// be creatable from some type T.
pub trait Creatable<T>
where
    Self: BytesEncodable,
{
    /// Create an instance of Self based on the provided input with type T.
    fn new(value: &T) -> Self;
}

/// Defines common method used to encode/serialize an instance of Self (`self`)
/// to a bytes vector.
pub trait Encodable<S>
where
    Self: Sized,
    S: Creatable<Self>,
{
    /// Encode `self` to a bytes vector.
    fn encode(&self) -> Vec<u8> {
        S::new(&self).encode_to_bytes()
    }
}

/// Decodes a slice of bytes to an instance of type S using additional payload data
/// required for the creation of S of type P.
pub trait Decoder<S, P>
where
    Self: BytesDecodable<S, P>,
{
    /// Decode the bytes data to an instance of type S using the payload of type P.
    fn decoder(data: &[u8], payload: P) -> Result<S, DecodeError> {
        Self::decode_from_bytes(data, payload)
    }
}

impl<P, S: BytesDecodable<E, P> + Creatable<E>, E: Encodable<S>> Decoder<E, P> for S {}

/// Defines the common interface for anything to be decodable/deserializable based on
/// using some payload.
pub trait Decodable<E>
where
    Self: Slicable,
{
    type Payload;
    type Latest: Decoder<E, Self::Payload>;

    fn decode(&self, payload: Self::Payload) -> Result<E, DecodeError> {
        Self::Latest::decoder(self.as_slice(), payload)
    }
}

impl Slicable for Vec<u8> {
    fn as_slice(&self) -> &[u8] {
        self.as_slice()
    }
}

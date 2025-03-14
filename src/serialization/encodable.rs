use super::utils::Slicable;

#[cfg(feature = "py")]
use pyo3::{exceptions::PyRuntimeError, PyErr};

#[derive(Debug, Clone)]
pub struct DecodeError {
    msg: String,
}

impl DecodeError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl std::error::Error for DecodeError {}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "decoding failed: {}", self.msg)
    }
}

#[cfg(feature = "py")]
impl std::convert::From<DecodeError> for PyErr {
    fn from(err: DecodeError) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}

impl std::convert::From<prost::DecodeError> for DecodeError {
    fn from(err: prost::DecodeError) -> DecodeError {
        DecodeError::new(err.to_string())
    }
}

pub trait BytesEncodable {
    fn encode_to_bytes(&self) -> Vec<u8>;
}

pub trait BytesDecodable<S, P = ()> {
    fn decode_from_bytes(bytes: &[u8], payload: P) -> Result<S, DecodeError>;
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
    fn encode(&self) -> Vec<u8> {
        S::new(&self).encode_to_bytes()
    }
}

pub trait Decoder<S, P>
where
    Self: BytesDecodable<S, P>,
{
    fn decoder(data: &[u8], payload: P) -> Result<S, DecodeError> {
        Self::decode_from_bytes(data, payload)
    }
}

impl<P, S: BytesDecodable<E, P> + Creatable<E>, E: Encodable<S>> Decoder<E, P> for S {}

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

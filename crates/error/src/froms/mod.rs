use std::io;

use crate::LunaModelError;

impl From<io::Error> for LunaModelError {
    fn from(value: io::Error) -> Self {
        LunaModelError::Compression(value.to_string().into())
    }
}

impl From<prost::DecodeError> for LunaModelError {
    fn from(err: prost::DecodeError) -> LunaModelError {
        LunaModelError::Decoding(err.to_string().into())
    }
}

impl From<std::fmt::Error> for LunaModelError {
    fn from(err: std::fmt::Error) -> Self {
        LunaModelError::Formatter(err.to_string().into())
    }
}

impl From<rand::distr::uniform::Error> for LunaModelError {
    fn from(value: rand::distr::uniform::Error) -> Self {
        LunaModelError::RandomSampling(value.to_string().into())
    }
}

impl From<strum::ParseError> for LunaModelError {
    fn from(value: strum::ParseError) -> Self {
        LunaModelError::Decoding(value.to_string().into())
    }
}

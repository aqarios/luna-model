use std::io;

use crate::LunaModelError;

impl From<io::Error> for LunaModelError {
    /// Maps I/O failures into the compression/decompression error category.
    fn from(value: io::Error) -> Self {
        LunaModelError::Compression(value.to_string().into())
    }
}

impl From<prost::DecodeError> for LunaModelError {
    /// Maps protobuf decode failures into the generic decoding error category.
    fn from(err: prost::DecodeError) -> LunaModelError {
        LunaModelError::Decoding(err.to_string().into())
    }
}

impl From<std::fmt::Error> for LunaModelError {
    /// Maps formatting failures into the formatter error category.
    fn from(err: std::fmt::Error) -> Self {
        LunaModelError::Formatter(err.to_string().into())
    }
}

impl From<rand::distr::uniform::Error> for LunaModelError {
    /// Maps random distribution setup failures into the random sampling category.
    fn from(value: rand::distr::uniform::Error) -> Self {
        LunaModelError::RandomSampling(value.to_string().into())
    }
}

impl From<strum::ParseError> for LunaModelError {
    /// Maps enum parse failures into the decoding error category.
    fn from(value: strum::ParseError) -> Self {
        LunaModelError::Decoding(value.to_string().into())
    }
}

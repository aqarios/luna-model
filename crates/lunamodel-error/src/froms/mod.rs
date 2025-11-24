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

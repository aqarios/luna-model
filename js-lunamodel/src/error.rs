use lunamodel::error::LunaModelError;
use napi::bindgen_prelude::{Error, Status};

pub(crate) fn map_luna_error(err: LunaModelError) -> Error {
    Error::new(map_luna_status(&err), err.to_string())
}

fn map_luna_status(err: &LunaModelError) -> Status {
    match err {
        LunaModelError::Compression(_) | LunaModelError::Decoding(_) => Status::InvalidArg,
        LunaModelError::Computation(_)
        | LunaModelError::Dtype(_)
        | LunaModelError::IndexOutOfBounds(_)
        | LunaModelError::InvalidTolerance(_)
        | LunaModelError::NoConstraintForKey(_)
        | LunaModelError::SampleIncompatibleVtype
        | LunaModelError::SampleIncorrectLength(_)
        | LunaModelError::SampleUnexpectedVariable(_)
        | LunaModelError::VariableNotExisting(_) => Status::InvalidArg,
        _ => Status::GenericFailure,
    }
}

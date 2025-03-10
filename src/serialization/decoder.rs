use prost::{DecodeError, Message};

use crate::core::{Model, VarId};

use super::{ser_model::SerializableModel, versioned::Versioned};

pub fn decode_model(data: &[u8]) -> Result<Model<VarId, f64>, DecodeError> {
    let versioned_model = Versioned::decode(data)?;
    // do something with the version in the future, i.e., depending on the version
    // choose a different decoding of ther internal bytes.
    match versioned_model.version {
        _ => SerializableModel::decoded(versioned_model.data.as_slice()),
    }
}

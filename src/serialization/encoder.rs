use crate::core::{Model, VarId};

use super::ser_model::SerializableModel;
use super::versioned::Versioned;

pub fn encode_model(model: &Model<VarId, f64>) -> Vec<u8> {
    Versioned::encoded(SerializableModel::new(&model))
}

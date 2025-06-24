use std::hash::{DefaultHasher, Hash, Hasher};

use super::hash_model::HashModel;
use crate::core::Model;

pub fn hash_model(model: &Model) -> u64 {
    let mut s = DefaultHasher::new();
    let ser = HashModel::build(&model);
    ser.hash(&mut s);
    s.finish()
}

//! Stable-ish model hashing helpers.
//!
//! This crate serializes selected model content into protobuf-backed byte
//! structures and then feeds those bytes into Rust's default hasher. The goal is
//! not cryptographic hashing; it is reproducible content fingerprints for
//! comparisons, caching, and test assertions.
use std::hash::{DefaultHasher, Hash, Hasher};

use lunamodel_core::Model;

mod constr;
mod env;
mod expr;
mod model;

use model::HashModel;

/// Hashes a model based on its encoded semantic content.
pub fn hash_model(model: &Model) -> u64 {
    let mut s = DefaultHasher::new();
    let hashmodel = HashModel::build(model);
    hashmodel.hash(&mut s);
    s.finish()
}

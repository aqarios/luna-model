//! Deterministic, order-independent content hashing for models.
//!
//! The hash is computed directly from a model's semantic content (variables,
//! expressions, constraints) rather than from its wire (de)serialization
//! format, so changes to the serializer have no effect on hash values.
//! Internal sparse storage order is never relied upon - every collection is
//! explicitly sorted by a stable key before hashing, so the result does not
//! depend on insertion order or on how a given internal representation
//! happens to iterate today. Cost is proportional to the model's actual
//! nonzero content (variables, constraints, and their nonzero terms), not to
//! the model's variable-ID space.
//!
//! The mixing algorithm (FxHash) is fixed by its crate implementation rather
//! than by the Rust standard library, so upgrading a Rust toolchain does not
//! silently change hash results the way `std::hash::DefaultHasher` could
//! (its algorithm is explicitly unspecified and may change between Rust
//! releases).

use std::hash::Hasher;

use lunamodel_core::Model;
use rustc_hash::FxHasher;

mod constr;
mod env;
mod expr;
mod model;
mod util;

/// Bump this whenever the hashing scheme itself changes, so that hashes
/// computed under different schemes can never silently collide.
const HASH_SCHEME_VERSION: u64 = 2;

/// Hashes a model based on its semantic content.
pub fn hash_model(m: &Model) -> u64 {
    let mut h = FxHasher::default();
    h.write_u64(HASH_SCHEME_VERSION);
    model::hash_model(m, &mut h);
    h.finish()
}

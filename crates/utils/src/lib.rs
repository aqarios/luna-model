//! Small iterator and defaults utilities shared across crates.
//!
//! The helpers in this crate stay intentionally minimal. They avoid pulling in a
//! heavier iterator utility dependency for the few deduplication behaviors that
//! LunaModel uses repeatedly when walking variables, types, and constraints.
mod cast;
mod cmp_tol;
mod iterator;

pub mod defaults;
pub use cast::{cast_near_integral, validate_tol};
pub use cmp_tol::{float_eq, float_ge, float_le};
pub use iterator::{unique, unique_by};

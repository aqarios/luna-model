//! Small iterator and defaults utilities shared across crates.
//!
//! The helpers in this crate stay intentionally minimal. They avoid pulling in a
//! heavier iterator utility dependency for the few deduplication behaviors that
//! LunaModel uses repeatedly when walking variables, types, and constraints.
pub mod defaults;
mod iterator;

pub use iterator::{UniqueIter, UniqueIterMap, unique, unique_by};

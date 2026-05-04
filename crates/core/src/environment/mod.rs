//! Variable environments.
//!
//! The environment owns the variables referenced by expressions and models.
//! Expressions and variable references do not store variable metadata directly;
//! they point back into an [`Environment`] or shared [`ArcEnv`] instead. This
//! keeps cloning cheap and lets multiple expressions refer to the same variable
//! set without duplicating state.

mod arcenv;
mod env;
mod util;

/// Shared, lock-protected environment wrapper used throughout the core crate.
pub use arcenv::ArcEnv;
/// Concrete environment that owns variables and lookup tables.
pub use env::Environment;

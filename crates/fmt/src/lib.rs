//! Formatting helpers for LunaModel domain objects.
//!
//! This crate centralizes developer-facing string and debug representations that
//! are shared by the Rust API and, when enabled, the Python bindings. The goal
//! is not serialization fidelity; it is readable, reasonably stable output for
//! REPLs, logs, and test assertions.
mod custom;
mod options;

mod bounds;
mod constr;
mod constr_coll;
mod env;
mod expr;
mod model;
pub mod sol;
mod specs;
mod timing;
mod transformation;
mod var;

pub use custom::{CustomFormat, CustomFormatWrapper};
pub use options::FormatOpt;

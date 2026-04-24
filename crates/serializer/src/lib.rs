//! Versioned binary encoding and decoding for LunaModel structures.
//!
//! This crate owns the workspace's stable-on-disk / over-the-wire encoding
//! story. The public API is intentionally trait-based so model, expression,
//! environment, solution, and transformation-record types can all share the same
//! compression and versioning pipeline.
mod compress;
mod encodables;
mod encode;
mod utils;
mod versionize;
mod versions;

pub mod prelude;

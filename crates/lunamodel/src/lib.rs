//! # LunaModel
//!
//! `lunamodel` is the top-level facade crate for the LunaModel workspace.
//! It does not define the modeling primitives itself. Instead, it re-exports the
//! workspace crates that make up the project so downstream users can depend on a
//! single crate and enable only the feature groups they need.
//!
//! The rough layering looks like this:
//!
//! - [`core`] contains the symbolic modeling primitives such as environments,
//!   variables, expressions, constraints, models, and solutions.
//! - [`types`], [`error`], and [`utils`] provide the shared building blocks used
//!   by the rest of the workspace.
//! - Optional crates such as `translate`, `transform`, `serializer`, and
//!   `python` add format conversion, model rewriting, persistence, and Python
//!   bindings on top of the core data structures.
//!
//! This crate is intentionally thin. When documenting or changing behavior,
//! the real implementation usually lives in one of the re-exported crates.
//! New contributors should usually start with [`core`] and only then follow
//! the optional crates that build on it.
//!
//! ## Feature Map
//!
//! The facade keeps the workspace feature structure explicit:
//!
//! - `hashing` enables the `hashing` re-export
//! - `io` enables the `io` re-export
//! - `python` enables the `python` re-export
//! - `serializer` enables the `serializer` re-export
//! - `transform` enables the `transform` re-export
//! - `transpiler` enables the `transpiler` re-export
//! - `translate` enables the `translate` re-export
//! - `unwind` enables the `unwind` re-export
//! - `py-io` enables the Python-oriented I/O feature set in `io`
//! - `py-types` enables Python-oriented conversions in `types`
//! - `py-error` enables Python-oriented conversions in `error`
//! - `full` enables all of the above feature groups
//!
//! The distinction matters because many internal crates and extension projects do
//! not need the full workspace. Pulling in only the required feature groups keeps
//! compile time down and avoids unnecessary optional dependencies.
//!
//! ## Internal Extension Development
//!
//! Internal Rust extension crates usually depend on `lunamodel` rather than
//! importing each workspace crate individually. That pattern gives extension
//! projects a stable entry point while still allowing them to opt into only the
//! feature sets they actually use.
//!
//! A typical dependency looks like this:
//!
//! ```toml
//! [dependencies]
//! luna-model = { path = "../luna-model/crates/lunamodel", default-features = false, features = ["translate", "transform"] }
//! ```
//!
//! In extension code, the most common imports then come from:
//!
//! - [`core`] for modeling primitives and operations
//! - [`types`] for shared enums and small domain types
//! - [`error`] for common result and error handling
//! - optional crates such as `translate` or `transform` when the extension
//!   needs those subsystems
//!
//! When you are building internal tooling or bindings, prefer re-export paths
//! through this facade unless you have a concrete reason to bind directly to a
//! lower-level crate. It keeps dependency declarations simpler and makes feature
//! intent visible in one place.

/// Core symbolic modeling types and operations.
pub use lunamodel_core as core;
/// Shared error types used throughout the workspace.
pub use lunamodel_error as error;
/// Shared small value types such as senses, variable types, and indices.
pub use lunamodel_types as types;
/// Shared helper utilities that do not belong to the domain crates directly.
pub use lunamodel_utils as utils;

#[cfg(feature = "hashing")]
/// Hashing helpers for content-based identity and cache-style workflows.
pub use lunamodel_hashing as hashing;

#[cfg(any(feature = "io", feature = "py-io"))]
/// Lower-level I/O helpers used by format adapters and bindings.
pub use lunamodel_io as io;

#[cfg(feature = "python")]
/// Rust-side implementation of the Python bindings.
pub use lunamodel_python as python;

#[cfg(feature = "serializer")]
/// Versioned model and solution serialization support.
pub use lunamodel_serializer as serializer;

#[cfg(feature = "transform")]
/// Built-in model transformation passes and pipelines.
pub use lunamodel_transform as transform;

#[cfg(feature = "transpiler")]
/// Infrastructure for composing and recording transformation-style workflows.
pub use lunamodel_transpiler as transpiler;

#[cfg(feature = "translate")]
/// Translators between LunaModel and external model formats.
pub use lunamodel_translate as translate;

#[cfg(feature = "unwind")]
/// Utilities for expanding or replaying reversible transformations.
pub use lunamodel_unwind as unwind;

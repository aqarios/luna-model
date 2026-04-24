//! Translators between LunaModel structures and external model formats.
#![allow(clippy::duplicate_mod)]
mod target;

pub mod model;

/// Supported translation targets surfaced by this crate.
pub use target::TranslationTarget;

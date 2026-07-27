//! Plugin-authoring macros: the C-boundary (Rust-backed) way to expose passes to Python.
//!
//! These generate the Python-facing pass classes from safe-Rust trait impls
//! (`rs_lunamodel::plugin::{TransformationPass, AnalysisPass}`) and wire up module registration.
//! They are the counterpart to writing passes in *pure Python* (subclassing luna-model's
//! `PyTransformationPass`/`PyAnalysisPass`) — both kinds coexist and can be mixed in one
//! `PassManager`.
//!
//! All macros are `#[macro_export]`, so downstream crates reach them at the crate root
//! (`rs_lunamodel::py_transform_plugin!`); `py/mod.rs` also re-exports them under the `py` module
//! (`rs_lunamodel::py::py_transform_plugin!`). Either path works regardless of this layout. One
//! macro per file:
//! - [`py_transform_plugin!`] — transformation passes (`transform`).
//! - [`py_analysis_plugin!`] — analysis passes (`analysis`).
//! - [`register_plugins!`] — the `__luna_register__` module hook for reversal handlers (`register`).

mod analysis;
mod register;
mod transform;

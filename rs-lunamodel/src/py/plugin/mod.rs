//! Plugin-authoring macros: the C-boundary (Rust-backed) way to expose passes to Python.
//!
//! These generate the Python-facing pass classes from safe-Rust trait impls
//! (`luna_model_ffi::plugin::{TransformationPass, AnalysisPass}`) and wire up module registration.
//! They are the counterpart to writing passes in *pure Python* (subclassing luna-model's
//! `PyTransformationPass`/`PyAnalysisPass`) — both kinds coexist and can be mixed in one
//! `PassManager`.
//!
//! All macros are `#[macro_export]`, so downstream crates reach them at the crate root
//! (`luna_model_ffi::py_transform_plugin!`); `py/mod.rs` also re-exports them under the `py` module
//! (`luna_model_ffi::py::py_transform_plugin!`). Either path works regardless of this layout. One
//! macro per file:
//! - [`py_transform_plugin!`] — transformation passes (`transform`).
//! - [`py_analysis_plugin!`] — analysis passes (`analysis`).
//! - [`py_composite_plugin!`] — composite passes (`composite`).
//! - [`py_meta_analysis_plugin!`] — meta-analysis passes (`meta`).
//! - [`py_control_flow_plugin!`] — control-flow passes (`control_flow`).
//! - [`register_plugins!`] — the `__luna_register__` module hook for reversal handlers (`register`).

mod analysis;
mod composite;
mod control_flow;
mod meta;
mod register;
mod transform;

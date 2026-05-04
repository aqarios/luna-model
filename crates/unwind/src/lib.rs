//! Re-exports for panic-to-Python unwinding helpers.
//!
//! The workspace keeps the proc-macro and runtime pieces split into separate
//! crates, and this facade crate is the import point used by the Python binding
//! layer.
/// Attribute macro that rewrites impl methods to return `PyResult` and execute through [`unwind`].
pub use lunamodel_unwindable::unwindable;
/// Runtime helper that catches Rust panics and converts them into Python exceptions.
pub use lunamodel_unwinder::unwind;

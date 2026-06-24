//! Stable C ABI for LunaModel.
//!
//! This crate exposes a `extern "C"` surface over the LunaModel core so that:
//!   1. C/C++ projects can link the `cdylib`/`staticlib` directly, and
//!   2. the Python wheel can publish the same surface as a `PyCapsule`
//!      (`luna_model._lm._C_API`) for in-process use by other native extensions.
//!
//! The boundary is C ABI on purpose: Rust has no stable ABI, so this is the only
//! safe way for independently compiled extensions to share one in-process instance.

pub mod buffer;
pub mod capi_table;
pub mod error;
pub mod model;
pub mod solution;
pub mod version;

/// ABI version of the C API table.
///
/// Bump this on any breaking change to the layout of the exported table struct
/// or to any exported function signature. Consumers must check it after import.
pub const CAPI_ABI_VERSION: u32 = 1;

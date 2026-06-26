//! Stable C ABI for LunaModel.
//!
//! The ABI types (`LunaModelCApi`, `LmStatus`, `LmBuffer`, the opaque handles, and
//! `CAPI_ABI_VERSION`) live in the dependency-free `lunamodel-cext-vtable` crate and
//! are re-exported here. Downstream Rust extensions that call the API through the
//! `luna_model._lm._C_API` capsule depend on that crate directly, so they never link
//! a second copy of the core.

pub use lunamodel_cext_vtable::{
    LmBuffer, LmEnv, LmModel, LmSense, LmSol, LmStatus, LmVref, LmVrefBuffer, LmVtype,
    LmVtypeBuffer, LunaModelCApi,
};

/// ABI version of the C API table.
///
/// Bump this on any breaking change to the layout of the exported table struct
/// or to any exported function signature. Consumers must check it after import.
//
// Defined here as a literal in the binding crate so cbindgen emits the
// `LUNA_MODEL_CAPI_ABI_VERSION` macro into the header. The vtable crate's copy
// (used by downstream Rust consumers) carries `cbindgen:ignore`; the assert keeps
// the two in lock-step.
pub const CAPI_ABI_VERSION: u32 = 1;
const _: () = assert!(CAPI_ABI_VERSION == lunamodel_cext_vtable::CAPI_ABI_VERSION);

pub mod buffer;
pub mod capi_table;
pub mod env;
pub mod error;
pub mod model;
pub mod solution;
pub mod version;

mod utils;

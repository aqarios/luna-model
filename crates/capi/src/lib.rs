//! ABI-only type definitions for the LunaModel C API.
//!
//! This crate has no dependency on the LunaModel core — it holds only the
//! `#[repr(C)]` types that make up the stable C ABI (the [`LunaModelCApi`] table,
//! [`LmStatus`], [`LmBuffer`], the opaque handles, and the ABI version). Both
//! `lunamodel-capi-host` (the implementation + filled table) and downstream Rust
//! extensions that call the API through the `luna_model._lm._C_API` capsule depend
//! on this crate for the shared types, so a consumer never links a second copy of
//! the core.
#![allow(rustdoc::broken_intra_doc_links)]

mod core;
mod vtable;

pub mod error;
pub mod plugin;

/// ABI version of the C API table.
///
/// Bump this on any breaking change to the layout of the exported table struct
/// or to any exported function signature. Consumers must check it after import.
///
/// cbindgen:ignore
pub const CAPI_ABI_VERSION: u32 = 2;

pub use core::{
    LmAssignment, LmBoolMapBuffer, LmBoolMapEntry, LmBound, LmBoundTag, LmBounds, LmBuffer, LmCC,
    LmCmpBuffer, LmColumn, LmColumnBuffer, LmComparator, LmConstraint, LmCtype, LmCtypeBuffer,
    LmEnv, LmEnvId, LmError, LmExpr, LmF64Buffer, LmHigherOrder, LmHigherOrderItem,
    LmHigherOrderItemBuffer, LmIndexMapItem, LmIndexMapItemBuffer, LmLazyBounds, LmLinear,
    LmLinearItem, LmLinearItemBuffer, LmModel, LmQuadratic, LmQuadraticItem, LmQuadraticItemBuffer,
    LmResultView, LmResultViewBuffer, LmSampleView, LmSampleViewBuffer, LmSense, LmSol,
    LmSolutionLookup, LmSpecs, LmStatus, LmStrBuffer, LmSubTimer, LmTimer, LmTiming, LmU32Buffer,
    LmValueSource, LmVar, LmVref, LmVrefBuffer, LmVtype, LmVtypeBuffer,
};

pub use vtable::LunaModelCApi;

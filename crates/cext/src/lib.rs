//! Stable C ABI for LunaModel.
//!
//! The ABI types (`LunaModelCApi`, `LmStatus`, `LmBuffer`, the opaque handles, and
//! `CAPI_ABI_VERSION`) live in the dependency-free `lunamodel-cext-vtable` crate and
//! are re-exported here. Downstream Rust extensions that call the API through the
//! `luna_model._lm._C_API` capsule depend on that crate directly, so they never link
//! a second copy of the core.

pub use lunamodel_cext_vtable::LunaModelCApi;
pub use lunamodel_cext_vtable::{
    LmAssignment, LmBoolMapBuffer, LmBoolMapEntry, LmBound, LmBoundTag, LmBounds, LmBuffer, LmCC,
    LmCmpBuffer, LmColumn, LmColumnBuffer, LmComparator, LmConstraint, LmCtype, LmCtypeBuffer,
    LmEnv, LmEnvId, LmExpr, LmF64Buffer, LmHigherOrder, LmHigherOrderItem, LmHigherOrderItemBuffer,
    LmIndexMapItem, LmIndexMapItemBuffer, LmLazyBounds, LmLinear, LmLinearItem, LmLinearItemBuffer,
    LmModel, LmQuadratic, LmQuadraticItem, LmQuadraticItemBuffer, LmResultView, LmResultViewBuffer,
    LmSampleView, LmSampleViewBuffer, LmSense, LmSol, LmSpecs, LmStatus, LmStrBuffer, LmTimer,
    LmTiming, LmU32Buffer, LmValueSource, LmVar, LmVref, LmVrefBuffer, LmVtype, LmVtypeBuffer,
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

pub mod plugin;

pub mod bound;
pub mod bounds;
pub mod buffer;
pub mod capi_table;
pub mod cc;
pub mod col;
pub mod constr;
pub mod env;
pub mod error;
mod eval;
pub mod expr;
pub mod higher_order;
pub mod lazy_bounds;
pub mod linear;
pub mod model;
pub mod quadratic;
pub mod res;
pub mod sample;
pub mod sol;
pub mod specs;
pub mod timer;
pub mod timing;
pub mod var;
pub mod version;
pub mod vref;

mod utils;

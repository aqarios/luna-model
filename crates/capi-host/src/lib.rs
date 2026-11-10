//! Stable C ABI for LunaModel.
//!
//! The ABI types (`LunaModelCApi`, `LmStatus`, `LmBuffer`, the opaque handles, and
//! `CAPI_ABI_VERSION`) live in the dependency-free `lunamodel-capi` crate and
//! are re-exported here. Downstream Rust extensions that call the API through the
//! `luna_model._lm._C_API` capsule depend on that crate directly, so they never link
//! a second copy of the core.

pub use lunamodel_capi::LunaModelCApi;
pub use lunamodel_capi::{
    LmAssignment, LmBoolMapBuffer, LmBoolMapEntry, LmBound, LmBoundTag, LmBounds, LmBuffer, LmCC,
    LmCmpBuffer, LmColumn, LmColumnBuffer, LmComparator, LmConstraint, LmCtype, LmCtypeBuffer,
    LmEnv, LmEnvId, LmError, LmExpr, LmF64Buffer, LmHigherOrder, LmHigherOrderItem,
    LmHigherOrderItemBuffer, LmIndexMapItem, LmIndexMapItemBuffer, LmLazyBounds, LmLinear,
    LmLinearItem, LmLinearItemBuffer, LmModel, LmQuadratic, LmQuadraticItem, LmQuadraticItemBuffer,
    LmResultView, LmResultViewBuffer, LmSampleView, LmSampleViewBuffer, LmSense, LmSol,
    LmSolutionLookup, LmSpecs, LmStatus, LmStrBuffer, LmSubTimer, LmTimer, LmTiming, LmU32Buffer,
    LmValueSource, LmVar, LmVref, LmVrefBuffer, LmVtype, LmVtypeBuffer,
};

/// ABI version of the C API table.
///
/// Bump this on any breaking change to the layout of the exported table struct
/// or to any exported function signature. Consumers must check it after import.
/// Appending entry points to the end of the table is not breaking: consumers check
/// the host table's size before calling one (`ci/tools/check-abi.py` enforces this).
//
// Defined here as a literal in the binding crate so cbindgen emits the
// `LUNA_MODEL_CAPI_ABI_VERSION` macro into the header. The vtable crate's copy
// (used by downstream Rust consumers) carries `cbindgen:ignore`; the assert keeps
// the two in lock-step.
pub const CAPI_ABI_VERSION: u32 = 1;
const _: () = assert!(CAPI_ABI_VERSION == lunamodel_capi::CAPI_ABI_VERSION);

// The `LM_MODEL_PART_*` flags, mirrored here for the same reason as `CAPI_ABI_VERSION`.
/// The objective expression.
pub const LM_MODEL_PART_OBJECTIVE: u32 = 1;
/// The constraint collection.
pub const LM_MODEL_PART_CONSTRAINTS: u32 = 2;
/// The environment: variables, their types and their bounds.
pub const LM_MODEL_PART_VARIABLES: u32 = 4;
/// The optimization sense.
pub const LM_MODEL_PART_SENSE: u32 = 8;
/// Every model part; the safe choice when a pass cannot say what it touches.
pub const LM_MODEL_PART_ALL: u32 = 15;
const _: () = {
    use lunamodel_capi::plugin as abi;
    assert!(LM_MODEL_PART_OBJECTIVE == abi::LM_MODEL_PART_OBJECTIVE);
    assert!(LM_MODEL_PART_CONSTRAINTS == abi::LM_MODEL_PART_CONSTRAINTS);
    assert!(LM_MODEL_PART_VARIABLES == abi::LM_MODEL_PART_VARIABLES);
    assert!(LM_MODEL_PART_SENSE == abi::LM_MODEL_PART_SENSE);
    assert!(LM_MODEL_PART_ALL == abi::LM_MODEL_PART_ALL);
};

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
pub mod sol_lookup;
pub mod specs;
pub mod timer;
pub mod timing;
pub mod var;
pub mod version;
pub mod vref;

mod utils;

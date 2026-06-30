//! ABI-only type definitions for the LunaModel C API.
//!
//! This crate has no dependency on the LunaModel core — it holds only the
//! `#[repr(C)]` types that make up the stable C ABI (the [`LunaModelCApi`] table,
//! [`LmStatus`], [`LmBuffer`], the opaque handles, and the ABI version). Both
//! `lunamodel-cext` (the implementation + filled table) and downstream Rust
//! extensions that call the API through the `luna_model._lm._C_API` capsule depend
//! on this crate for the shared types, so a consumer never links a second copy of
//! the core. Mirrors Qiskit's `qiskit-cext-vtable` (without the slot machinery).
#![allow(rustdoc::broken_intra_doc_links)]

use std::os::raw::c_char;

/// ABI version of the C API table.
///
/// Bump this on any breaking change to the layout of the exported table struct
/// or to any exported function signature. Consumers must check it after import.
///
/// cbindgen:ignore
pub const CAPI_ABI_VERSION: u32 = 1;

/// Status codes returned by every fallible `lm_*` entry point.
///
/// `Ok` is guaranteed to be `0`. On any non-`Ok` status, a human-readable
/// description is available from [`lm_last_error_message`] until the next C API
/// call on the same thread.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LmStatus {
    /// Operation succeeded.
    Ok = 0,
    /// A required pointer argument was null.
    NullPointer = 1,
    /// An argument was invalid (e.g. non-UTF-8 bytes where text was required).
    InvalidArgument = 2,
    /// Encoding a value to bytes failed.
    Serialization = 3,
    /// Decoding a value from bytes failed.
    Deserialization = 4,
    /// A Rust panic was caught at the FFI boundary.
    Panic = 5,
    /// An error was encountered.
    Errored = 6,
}

/// Objective optimization sense used by the C API.
///
/// This is the C-ABI-stable mirror of LunaModel's Rust `Sense` enum.
/// Use this when creating or updating a model through `extern "C"` functions.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LmSense {
    /// Minimize the model objective.
    Min = 0,

    /// Maximize the model objective.
    Max = 1,
}

/// Vtype used by the C API.
///
/// This is the C-ABI-stable mirror of LunaModel's Rust `Vtype` enum.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LmVtype {
    /// A binary variable that can take values 0 or 1.
    Binary = 0,
    /// An inverted binary variable (`!b == 1 - b`) that is not optimized itself and it's value
    /// depends on the value of the corresponding [Vtype::Binary] variable.
    InvertedBinary = 1,
    /// Spin variable that can take values -1 or +1.
    Spin = 2,
    /// Discrete integer-valued variable that take integer values within bounds.
    Integer = 3,
    /// Continuous real-valued variable that take any value within given bounds.
    Real = 4,
}

/// Bound used by the C API.
///
/// One side of a variable bound interval.
///
/// This is the C-ABI-stable mirror of LunaModel's Rust `Bound` enum.
/// `tag == LmBoundTag_Bounded` means `value` contains the finite inclusive
/// bound value. `tag == LmBoundTag_Unbounded` means `value` is ignored.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LmBound {
    pub tag: LmBoundTag,
    pub value: f64,
}

/// Discriminant for `LmBound`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LmBoundTag {
    /// A finite inclusive bound value.
    Bounded = 0,

    /// No bound on this side of the interval.
    Unbounded = 1,
}

/// Comparator used by the C API.
///
/// This is the C-ABI-stable mirror of LunaModel's Rust `Comparator` enum.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LmComparator {
    /// The Equality comparison (==) for a constraint where LHS == RHS.
    Eq = 0,
    /// The Less-than or equal comparison (<=) for a constraint where LHS <= RHS.
    Le = 1,
    /// The Greater-than or equal comparison (>=) for a constraint where LHS >= RHS.
    Ge = 2,
}

/// Ctype used by the C API.
///
/// This is the C-ABI-stable mirror of LunaModel's Rust `Ctype` enum.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LmCtype {
    /// Only equality constraints allowed.
    Unconstrained = 0,
    /// Only equality constraints allowed.
    Equality = 1,
    /// Only inequality constraints allowed.
    Inequality = 2,
    /// Only ge-inequality constraints allowed.
    LessEqual = 3,
    /// Only le-inequality constraints allowed.
    GreaterEqual = 4,
}

/// ValueSource used by the C API.
///
/// This is the C-ABI-stable mirror of LunaModel's Rust `ValueSource` enum.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LmValueSource {
    /// Use the raw solver-provided energies.
    Raw = 0,
    /// Use the model-evaluated objective values.
    Obj = 1,
}

/// An owned, heap-allocated byte buffer the caller takes ownership of.
///
/// Buffers returned by LunaModel (e.g. from `lm_model_serialize`) must be released
/// with [`lm_buffer_free`]. The fields mirror a `Vec<u8>` so it can be rebuilt for
/// deallocation.
#[repr(C)]
pub struct LmBuffer {
    /// Pointer to the first byte, or null for an empty buffer.
    pub data: *mut u8,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// An owned, heap-allocated Vtype buffer the caller takes ownership of.
///
/// Vtype buffers returned by LunaModel must be released
/// with [`lm_vtype_buffer_free`]. The fields mirror a `Vec<Vtype>` so it can
/// be rebuilt for deallocation.
#[repr(C)]
pub struct LmVtypeBuffer {
    /// Pointer to the first Vtype, or null for an empty buffer.
    pub data: *mut LmVtype,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// An owned, heap-allocated Ctype buffer the caller takes ownership of.
///
/// Ctype buffers returned by LunaModel must be released
/// with [`lm_ctype_buffer_free`]. The fields mirror a `Vec<Ctype>` so it can
/// be rebuilt for deallocation.
#[repr(C)]
pub struct LmCtypeBuffer {
    /// Pointer to the first Ctype, or null for an empty buffer.
    pub data: *mut LmCtype,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// An owned, heap-allocated Comparator buffer the caller takes ownership of.
///
/// Comparator buffers returned by LunaModel must be released
/// with [`lm_cmp_buffer_free`]. The fields mirror a `Vec<Comparator>` so it can
/// be rebuilt for deallocation.
#[repr(C)]
pub struct LmCmpBuffer {
    /// Pointer to the first LmComparator, or null for an empty buffer.
    pub data: *mut LmComparator,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// An owned, heap-allocated VarRef buffer the caller takes ownership of.
///
/// Vtype buffers returned by LunaModel must be released
/// with [`lm_vref_buffer_free`]. The fields mirror a `Vec<VarRef>` so it can
/// be rebuilt for deallocation.
#[repr(C)]
pub struct LmVrefBuffer {
    /// Pointer to the first VarRef, or null for an empty buffer.
    pub data: *mut *mut LmVref,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// Opaque handle to a LunaModel model.
///
/// Created by [`lm_model_default`], [`lm_model_new`] or [`lm_model_deserialize`] and released with
/// [`lm_model_free`]. The contents are intentionally hidden from C.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmModel {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel solution.
///
/// Solutions are produced elsewhere (a solver or the Python API) and loaded here
/// from serialized bytes with [`lm_sol_deserialize`]. Release with
/// [`lm_sol_free`].
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmSol {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel environment.
///
/// Created by [`lm_env_default`] and released with
/// [`lm_env_free`]. The contents are intentionally hidden from C.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmEnv {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel variable reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmVref {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel bounds reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmBounds {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel timing reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmTiming {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel timer reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmTimer {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel variable reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmVar {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel lazy bounds reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmLazyBounds {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel constraint reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmConstraint {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel constraint-collection reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmCC {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel expression reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmExpr {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel linear terms reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmLinearTerm {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel quadratic terms reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmQuadraticTerm {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel higher-order terms reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmHigherOrderTerm {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel model-specs reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmSpecs {
    _private: [u8; 0],
}

/// Stable, versioned table of LunaModel C API entry points.
///
/// Consumers obtaining this via the capsule must check [`Self::abi_version`]
/// against the `CAPI_ABI_VERSION` they were compiled against before calling.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LunaModelCApi {
    /// ABI version of this table (`CAPI_ABI_VERSION` at build time).
    pub abi_version: u32,
    /// See [`crate::version::lm_version`].
    pub version: extern "C" fn() -> *const c_char,
    /// See [`crate::error::lm_last_error_message`].
    pub last_error_message: extern "C" fn() -> *const c_char,
    /// See [`crate::buffer::lm_buffer_free`].
    pub buffer_free: unsafe extern "C" fn(LmBuffer),
    /// See [`crate::buffer::lm_vtype_buffer_free`].
    pub vtype_buffer_free: unsafe extern "C" fn(LmVtypeBuffer),
    /// See [`crate::buffer::lm_ctype_buffer_free`].
    pub ctype_buffer_free: unsafe extern "C" fn(LmCtypeBuffer),
    /// See [`crate::buffer::lm_cmp_buffer_free`].
    pub cmp_buffer_free: unsafe extern "C" fn(LmCmpBuffer),
    /// See [`crate::buffer::lm_vref_buffer_free`].
    pub vref_buffer_free: unsafe extern "C" fn(LmVrefBuffer),
    // ///// //
    // MODEL //
    // ///// //
    /// See [`crate::model::lm_model_free`].
    pub model_free: unsafe extern "C" fn(*mut LmModel),
    /// See [`crate::model::lm_model_default`].
    pub model_default: extern "C" fn() -> *mut LmModel,
    /// See [`crate::model::lm_model_new`].
    pub model_new:
        unsafe extern "C" fn(*const c_char, *const LmSense, *mut *mut LmModel) -> LmStatus,
    pub model_with_env: unsafe extern "C" fn(
        *const c_char,
        *const LmSense,
        *const LmEnv,
        *mut *mut LmModel,
    ) -> LmStatus,
    /// See [`crate::model::lm_model_clone`].
    pub model_clone: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    /// See [`crate::model::lm_model_deep_clone`].
    pub model_deep_clone: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    /// See [`crate::model::lm_model_name`].
    pub model_name: unsafe extern "C" fn(*const LmModel, *mut LmBuffer) -> LmStatus,
    /// See [`crate::model::lm_model_sense`].
    pub model_sense: unsafe extern "C" fn(*const LmModel, *mut LmSense) -> LmStatus,
    /// See [`crate::model::lm_model_env`].
    pub model_env: unsafe extern "C" fn(*const LmModel, *mut *mut LmEnv) -> LmStatus,
    /// See [`crate::model::lm_model_specs`].
    pub model_specs: unsafe extern "C" fn(*const LmModel, *mut *mut LmSpecs) -> LmStatus,
    /// See [`crate::model::lm_model_vtypes`].
    pub model_vtypes: unsafe extern "C" fn(*const LmModel, *mut LmVtypeBuffer) -> LmStatus,
    /// See [`crate::model::lm_model_num_variables`].
    pub model_num_variables: unsafe extern "C" fn(*const LmModel, *mut usize) -> LmStatus,
    /// See [`crate::model::lm_model_vars`].
    pub model_vars: unsafe extern "C" fn(*const LmModel, *mut LmVrefBuffer) -> LmStatus,
    /// See [`crate::model::lm_model_var`].
    pub model_var:
        unsafe extern "C" fn(*const LmModel, *const c_char, *mut *mut LmVref) -> LmStatus,
    /// See [`crate::model::lm_model_objective`].
    pub model_objective: unsafe extern "C" fn(*const LmModel, *mut *mut LmExpr) -> LmStatus,
    /// See [`crate::model::lm_model_set_name`].
    pub model_set_name: unsafe extern "C" fn(*mut LmModel, *const c_char) -> LmStatus,
    /// See [`crate::model::lm_model_set_sense`].
    pub model_set_sense: unsafe extern "C" fn(*mut LmModel, *const LmSense) -> LmStatus,
    /// See [`crate::model::lm_model_set_objective`].
    pub model_set_objective: unsafe extern "C" fn(*mut LmModel, *const LmExpr) -> LmStatus,
    /// See [`crate::model::lm_model_add_var`].
    pub model_add_var: unsafe extern "C" fn(
        *mut LmModel,
        *const c_char,
        *const LmVtype,
        *const LmBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    /// See [`crate::model::lm_model_add_var_with_fallback`].
    pub model_add_var_with_fallback: unsafe extern "C" fn(
        *mut LmModel,
        *const c_char,
        *const LmVtype,
        *const LmBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    /// See [`crate::model::lm_model_satisfies`].
    pub model_satisfies:
        unsafe extern "C" fn(*const LmModel, *const LmSpecs, *mut bool) -> LmStatus,
    /// See [`crate::model::lm_model_serialize`].
    pub model_serialize: unsafe extern "C" fn(*const LmModel, *mut LmBuffer) -> LmStatus,
    /// See [`crate::model::lm_model_deserialize`].
    pub model_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmModel) -> LmStatus,
    /// See [`crate::model::lm_model_eq`].
    pub model_eq: unsafe extern "C" fn(*const LmModel, *const LmModel, *mut bool) -> LmStatus,
    /// See [`crate::model::lm_model_equal_contents`].
    pub model_equal_contents:
        unsafe extern "C" fn(*const LmModel, *const LmModel, *mut bool) -> LmStatus,
    /// See [`crate::model::lm_model_evaluate_solution`].
    pub model_evaluate_solution:
        unsafe extern "C" fn(*const LmModel, *const LmSol, *mut *mut LmSol) -> LmStatus,
    /// See [`crate::model::lm_model_evaluate_solution_with_tol`].
    pub model_evaluate_solution_with_tol:
        unsafe extern "C" fn(*const LmModel, *const LmSol, *const f64, *mut *mut LmSol) -> LmStatus,
    /// See [`crate::model::lm_model_substitute`].
    pub model_substitute:
        unsafe extern "C" fn(*mut LmModel, *const LmVref, *const LmExpr) -> LmStatus,
    // /////////// //
    // ENVIRONMENT //
    // /////////// //
    /// See [`crate::env::lm_env_free`].
    pub env_free: unsafe extern "C" fn(*mut LmEnv),
    /// See [`crate::env::lm_env_default`].
    pub env_default: extern "C" fn() -> *mut LmEnv,
    /// See [`crate::env::lm_env_clone`].
    pub env_clone: unsafe extern "C" fn(*const LmEnv, *mut *mut LmEnv) -> LmStatus,
    /// See [`crate::env::lm_env_equal_contents`].
    pub env_equal_contents: unsafe extern "C" fn(*const LmEnv, *const LmEnv, *mut bool) -> LmStatus,
    /// See [`crate::env::lm_env_deep_clone`].
    pub env_deep_clone: unsafe extern "C" fn(*const LmEnv, *mut *mut LmEnv) -> LmStatus,
    /// See [`crate::env::lm_env_len`].
    pub env_len: unsafe extern "C" fn(*const LmEnv, *mut usize) -> LmStatus,
    /// See [`crate::env::lm_env_id`].
    pub env_id: unsafe extern "C" fn(*const LmEnv, *mut u128) -> LmStatus,
    /// See [`crate::env::lm_env_is_empty`].
    pub env_is_empty: unsafe extern "C" fn(*const LmEnv, *mut bool) -> LmStatus,
    /// See [`crate::env::lm_env_vars`].
    pub env_vars: unsafe extern "C" fn(*const LmEnv, *mut LmVrefBuffer) -> LmStatus,
    /// See [`crate::env::lm_env_insert`].
    pub env_insert: unsafe extern "C" fn(
        *mut LmEnv,
        *const c_char,
        *const LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    /// See [`crate::env::lm_env_insert_with_fallback`].
    pub env_insert_with_fallback: unsafe extern "C" fn(
        *mut LmEnv,
        *const c_char,
        *const LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    /// See [`crate::env::lm_env_insert_inverted`].
    pub env_insert_inverted:
        unsafe extern "C" fn(*mut LmEnv, *const LmVref, *mut *mut LmVref) -> LmStatus,
    /// See [`crate::env::lm_env_remove`].
    pub env_remove: unsafe extern "C" fn(*mut LmEnv, *const LmVref) -> LmStatus,
    /// See [`crate::env::lm_env_lookup`].
    pub env_lookup: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut *mut LmVref) -> LmStatus,
    /// See [`crate::env::lm_env_get`].
    pub env_get: unsafe extern "C" fn(*const LmEnv, u32, *mut *mut LmVref) -> LmStatus,
    /// See [`crate::env::lm_env_contains`].
    pub env_contains: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut bool) -> LmStatus,
    /// See [`crate::env::lm_env_vtype_of`].
    pub env_vtype_of: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut LmVtype) -> LmStatus,
    /// See [`crate::env::lm_env_serialize`].
    pub env_serialize: unsafe extern "C" fn(*const LmEnv, *mut LmBuffer) -> LmStatus,
    /// See [`crate::env::lm_env_deserialize`].
    pub env_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmEnv) -> LmStatus,
    // //////// //
    // SOLUTION //
    // //////// //
    /// See [`crate::solution::lm_sol_free`].
    pub sol_free: unsafe extern "C" fn(*mut LmSol),
    /// See [`crate::solution::lm_sol_deserialize`].
    pub sol_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmSol) -> LmStatus,
    // ////////// //
    // EXPRESSION //
    // ////////// //
    /// See [`crate::expr::lm_expr_free`].
    pub expr_free: unsafe extern "C" fn(*mut LmExpr),
    /// See [`crate::expr::lm_expr_default`].
    pub expr_default: extern "C" fn() -> *mut LmExpr,
    /// See [`crate::expr::lm_expr_new`].
    pub expr_new: unsafe extern "C" fn(*const LmEnv, *mut *mut LmExpr) -> LmStatus,
    /// See [`crate::expr::lm_expr_new_constant`].
    pub expr_new_constant: unsafe extern "C" fn(*const LmEnv, f64, *mut *mut LmExpr) -> LmStatus,
    /// See [`crate::expr::lm_expr_clone`].
    pub expr_clone: unsafe extern "C" fn(*const LmExpr, *mut *mut LmExpr) -> LmStatus,
    /// See [`crate::expr::lm_expr_clone`].
    pub expr_deep_clone:
        unsafe extern "C" fn(*const LmExpr, *const LmEnv, *mut *mut LmExpr) -> LmStatus,
    /// See [`crate::expr::lm_expr_serialize`].
    pub expr_serialize: unsafe extern "C" fn(*const LmExpr, *mut LmBuffer) -> LmStatus,
    /// See [`crate::expr::lm_expr_deserialize`].
    pub expr_deserialize:
        unsafe extern "C" fn(*const u8, usize, *const LmEnv, *mut *mut LmExpr) -> LmStatus,
    /// See [`crate::expr::lm_expr_env`].
    pub expr_env: unsafe extern "C" fn(*const LmExpr, *mut *mut LmEnv) -> LmStatus,
    /// See [`crate::expr::lm_expr_offset`].
    pub expr_offset: unsafe extern "C" fn(*const LmExpr, *mut f64) -> LmStatus,
    /// See [`crate::expr::lm_expr_linear`].
    pub expr_linear: unsafe extern "C" fn(*const LmExpr, *mut *mut LmLinearTerm) -> LmStatus,
    /// See [`crate::expr::lm_expr_quadratic`].
    pub expr_quadratic: unsafe extern "C" fn(*const LmExpr, *mut *mut LmQuadraticTerm) -> LmStatus,
    /// See [`crate::expr::lm_expr_higher_order`].
    pub expr_higher_order:
        unsafe extern "C" fn(*const LmExpr, *mut *mut LmHigherOrderTerm) -> LmStatus,
    /// See [`crate::expr::lm_expr_num_vars`].
    pub expr_num_vars: unsafe extern "C" fn(*const LmExpr, *mut usize) -> LmStatus,
    /// See [`crate::expr::lm_expr_vtypes`].
    pub expr_vtypes: unsafe extern "C" fn(*const LmExpr, *mut LmVtypeBuffer) -> LmStatus,
    /// See [`crate::expr::lm_expr_vars`].
    pub expr_vars: unsafe extern "C" fn(*const LmExpr, *mut LmVrefBuffer) -> LmStatus,
    /// See [`crate::expr::lm_expr_complete_vars`].
    pub expr_complete_vars: unsafe extern "C" fn(*const LmExpr, *mut LmVrefBuffer) -> LmStatus,
    /// See [`crate::expr::lm_expr_degree`].
    pub expr_degree: unsafe extern "C" fn(*const LmExpr, *mut usize) -> LmStatus,
    /// See [`crate::expr::lm_expr_is_constant`].
    pub expr_is_constant: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    /// See [`crate::expr::lm_expr_has_quadratic`].
    pub expr_has_quadratic: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    /// See [`crate::expr::lm_expr_has_higher_order`].
    pub expr_has_higher_order: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    /// See [`crate::expr::lm_expr_eq`].
    pub expr_eq: unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut bool) -> LmStatus,
    /// See [`crate::expr::lm_expr_equal_contents`].
    pub expr_equal_contents:
        unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut bool) -> LmStatus,
    // //////////////////// //
    // ConstraintCollection //
    // //////////////////// //
    /// See [`crate::env::lm_cc_free`].
    pub cc_free: unsafe extern "C" fn(*mut LmCC),
    /// See [`crate::env::lm_cc_default`].
    pub cc_default: extern "C" fn() -> *mut LmCC,
    /// See [`crate::env::lm_cc_new`].
    pub cc_new: extern "C" fn() -> *mut LmCC,
    /// See [`crate::env::lm_cc_clone`].
    pub cc_clone: unsafe extern "C" fn(*const LmCC, *mut *mut LmCC) -> LmStatus,
    /// See [`crate::env::lm_cc_deep_clone`].
    pub cc_deep_clone: unsafe extern "C" fn(*const LmCC, *const LmEnv, *mut *mut LmCC) -> LmStatus,
    /// See [`crate::env::lm_cc_eq`].
    pub cc_eq: unsafe extern "C" fn(*const LmCC, *const LmCC, *mut bool) -> LmStatus,
    /// See [`crate::env::lm_cc_equal_contents`].
    pub cc_equal_contents: unsafe extern "C" fn(*const LmCC, *const LmCC, *mut bool) -> LmStatus,
    /// See [`crate::env::lm_cc_vtypes`].
    pub cc_vtypes: unsafe extern "C" fn(*const LmCC, *mut LmVtypeBuffer) -> LmStatus,
    /// See [`crate::env::lm_cc_ctypes`].
    pub cc_ctypes: unsafe extern "C" fn(*const LmCC, *mut LmCmpBuffer) -> LmStatus,
    /// See [`crate::env::lm_cc_vars`].
    pub cc_vars: unsafe extern "C" fn(*const LmCC, *mut LmVrefBuffer) -> LmStatus,
    /// See [`crate::env::lm_cc_complete_vars`].
    pub cc_complete_vars: unsafe extern "C" fn(*const LmCC, *mut LmVrefBuffer) -> LmStatus,
    /// See [`crate::env::lm_cc_is_empty`].
    pub cc_is_empty: unsafe extern "C" fn(*const LmCC, *mut bool) -> LmStatus,
    /// See [`crate::env::lm_cc_len`].
    pub cc_len: unsafe extern "C" fn(*const LmCC, *mut usize) -> LmStatus,
    /// See [`crate::env::lm_cc_get`].
    pub cc_get:
        unsafe extern "C" fn(*const LmCC, *const c_char, *mut *mut LmConstraint) -> LmStatus,
    /// See [`crate::env::lm_cc_add_constraint`].
    pub cc_add_constraint: unsafe extern "C" fn(
        *mut LmCC,
        *const LmConstraint,
        *const c_char,
        *mut LmBuffer,
    ) -> LmStatus,
    /// See [`crate::env::lm_cc_add_collection`].
    pub cc_add_collection: unsafe extern "C" fn(*mut LmCC, *const LmCC, *const c_char) -> LmStatus,
    /// See [`crate::env::lm_cc_set_constraint`].
    pub cc_set_constraint:
        unsafe extern "C" fn(*mut LmCC, *const c_char, *const LmConstraint) -> LmStatus,
    /// See [`crate::env::lm_cc_remove_constraint`].
    pub cc_remove_constraint: unsafe extern "C" fn(*mut LmCC, *const c_char) -> LmStatus,
    /// See [`crate::env::lm_cc_substitute`].
    pub cc_substitute: unsafe extern "C" fn(*mut LmCC, *const LmVref, *const LmExpr) -> LmStatus,
    /// See [`crate::cc::lm_cc_serialize`].
    pub cc_serialize: unsafe extern "C" fn(*const LmCC, *mut LmBuffer) -> LmStatus,
    /// See [`crate::cc::lm_cc_deserialize`].
    pub cc_deserialize:
        unsafe extern "C" fn(*const u8, usize, *const LmEnv, *mut *mut LmCC) -> LmStatus,
    // ////// //
    // VarRef //
    // ////// //
    /// See [`crate::env::lm_vref_free`].
    pub vref_free: unsafe extern "C" fn(*mut LmVref),
    /// See [`crate::env::lm_vref_new`].
    pub vref_new: unsafe extern "C" fn(u32, *const LmEnv, *mut *mut LmVref) -> LmStatus,
    /// See [`crate::env::lm_vref_id`].
    pub vref_id: unsafe extern "C" fn(*const LmVref, *mut u32) -> LmStatus,
    /// See [`crate::env::lm_vref_check_living`].
    pub vref_check_living: unsafe extern "C" fn(*const LmVref, *mut bool) -> LmStatus,
    /// See [`crate::env::lm_vref_name`].
    pub vref_name: unsafe extern "C" fn(*const LmVref, *mut LmBuffer) -> LmStatus,
    /// See [`crate::env::lm_vref_vtype`].
    pub vref_vtype: unsafe extern "C" fn(*const LmVref, *mut LmVtype) -> LmStatus,
    /// See [`crate::env::lm_vref_bounds`].
    pub vref_bounds: unsafe extern "C" fn(*const LmVref, *mut *mut LmBounds) -> LmStatus,
    /// See [`crate::env::lm_vref_inverted`].
    pub vref_inverted: unsafe extern "C" fn(*const LmVref, *mut *mut LmVref) -> LmStatus,
    /// See [`crate::env::lm_vref_hash`].
    pub vref_hash: unsafe extern "C" fn(*const LmVref, *mut u64) -> LmStatus,
    /// See [`crate::env::lm_vref_eq`].
    pub vref_eq: unsafe extern "C" fn(*const LmVref, *const LmVref, *mut bool) -> LmStatus,
    /// See [`crate::env::lm_vref_to_var`].
    pub vref_to_var: unsafe extern "C" fn(*const LmVref, *mut *mut LmVar) -> LmStatus,
    // //////// //
    // Variable //
    // //////// //
    /// See [`crate::env::lm_var_free`].
    pub var_free: unsafe extern "C" fn(*mut LmVar),
    /// See [`crate::env::lm_var_name`].
    pub var_name: unsafe extern "C" fn(*const LmVar, *mut LmBuffer) -> LmStatus,
    /// See [`crate::env::lm_var_vtype`].
    pub var_vtype: unsafe extern "C" fn(*const LmVar, *mut LmVtype) -> LmStatus,
    /// See [`crate::env::lm_var_bounds`].
    pub var_bounds: unsafe extern "C" fn(*const LmVar, *mut *mut LmBounds) -> LmStatus,
    /// See [`crate::env::lm_var_inverted`].
    pub var_inverted: unsafe extern "C" fn(*const LmVar, *mut u32, *mut bool) -> LmStatus,
}

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
    /// A requested optional value was not present.
    Absent = 7,
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

/// A 128-bit environment identity token, split into two 64-bit halves because C
/// has no native 128-bit integer type. Use it only for identity comparison
/// (compare both fields); do not do arithmetic on the halves.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LmEnvId {
    /// High 64 bits.
    pub hi: u64,
    /// Low 64 bits.
    pub lo: u64,
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
    pub version: extern "C" fn() -> *const c_char,
    pub last_error_message: extern "C" fn() -> *const c_char,
    pub buffer_free: unsafe extern "C" fn(LmBuffer),
    pub vtype_buffer_free: unsafe extern "C" fn(LmVtypeBuffer),
    pub ctype_buffer_free: unsafe extern "C" fn(LmCtypeBuffer),
    pub cmp_buffer_free: unsafe extern "C" fn(LmCmpBuffer),
    pub vref_buffer_free: unsafe extern "C" fn(LmVrefBuffer),
    // ///// //
    // MODEL //
    // ///// //
    pub model_free: unsafe extern "C" fn(*mut LmModel),
    pub model_default: extern "C" fn() -> *mut LmModel,
    pub model_new:
        unsafe extern "C" fn(*const c_char, *const LmSense, *mut *mut LmModel) -> LmStatus,
    pub model_with_env: unsafe extern "C" fn(
        *const c_char,
        *const LmSense,
        *const LmEnv,
        *mut *mut LmModel,
    ) -> LmStatus,
    pub model_clone: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    pub model_deep_clone: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    pub model_name: unsafe extern "C" fn(*const LmModel, *mut LmBuffer) -> LmStatus,
    pub model_sense: unsafe extern "C" fn(*const LmModel, *mut LmSense) -> LmStatus,
    pub model_env: unsafe extern "C" fn(*const LmModel, *mut *mut LmEnv) -> LmStatus,
    pub model_specs: unsafe extern "C" fn(*const LmModel, *mut *mut LmSpecs) -> LmStatus,
    pub model_vtypes: unsafe extern "C" fn(*const LmModel, *mut LmVtypeBuffer) -> LmStatus,
    pub model_num_variables: unsafe extern "C" fn(*const LmModel, *mut usize) -> LmStatus,
    pub model_vars: unsafe extern "C" fn(*const LmModel, *mut LmVrefBuffer) -> LmStatus,
    pub model_var:
        unsafe extern "C" fn(*const LmModel, *const c_char, *mut *mut LmVref) -> LmStatus,
    pub model_objective: unsafe extern "C" fn(*const LmModel, *mut *mut LmExpr) -> LmStatus,
    pub model_set_name: unsafe extern "C" fn(*mut LmModel, *const c_char) -> LmStatus,
    pub model_set_sense: unsafe extern "C" fn(*mut LmModel, *const LmSense) -> LmStatus,
    pub model_set_objective: unsafe extern "C" fn(*mut LmModel, *const LmExpr) -> LmStatus,
    pub model_add_var: unsafe extern "C" fn(
        *mut LmModel,
        *const c_char,
        *const LmVtype,
        *const LmBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub model_add_var_with_fallback: unsafe extern "C" fn(
        *mut LmModel,
        *const c_char,
        *const LmVtype,
        *const LmBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub model_satisfies:
        unsafe extern "C" fn(*const LmModel, *const LmSpecs, *mut bool) -> LmStatus,
    pub model_serialize: unsafe extern "C" fn(*const LmModel, *mut LmBuffer) -> LmStatus,
    pub model_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmModel) -> LmStatus,
    pub model_eq: unsafe extern "C" fn(*const LmModel, *const LmModel, *mut bool) -> LmStatus,
    pub model_equal_contents:
        unsafe extern "C" fn(*const LmModel, *const LmModel, *mut bool) -> LmStatus,
    pub model_evaluate_solution:
        unsafe extern "C" fn(*const LmModel, *const LmSol, *mut *mut LmSol) -> LmStatus,
    pub model_evaluate_solution_with_tol:
        unsafe extern "C" fn(*const LmModel, *const LmSol, *const f64, *mut *mut LmSol) -> LmStatus,
    pub model_substitute:
        unsafe extern "C" fn(*mut LmModel, *const LmVref, *const LmExpr) -> LmStatus,
    // /////////// //
    // ENVIRONMENT //
    // /////////// //
    pub env_free: unsafe extern "C" fn(*mut LmEnv),
    pub env_default: extern "C" fn() -> *mut LmEnv,
    pub env_clone: unsafe extern "C" fn(*const LmEnv, *mut *mut LmEnv) -> LmStatus,
    pub env_equal_contents: unsafe extern "C" fn(*const LmEnv, *const LmEnv, *mut bool) -> LmStatus,
    pub env_deep_clone: unsafe extern "C" fn(*const LmEnv, *mut *mut LmEnv) -> LmStatus,
    pub env_len: unsafe extern "C" fn(*const LmEnv, *mut usize) -> LmStatus,
    pub env_id: unsafe extern "C" fn(*const LmEnv, *mut LmEnvId) -> LmStatus,
    pub env_is_empty: unsafe extern "C" fn(*const LmEnv, *mut bool) -> LmStatus,
    pub env_vars: unsafe extern "C" fn(*const LmEnv, *mut LmVrefBuffer) -> LmStatus,
    pub env_insert: unsafe extern "C" fn(
        *mut LmEnv,
        *const c_char,
        *const LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub env_insert_with_fallback: unsafe extern "C" fn(
        *mut LmEnv,
        *const c_char,
        *const LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub env_insert_inverted:
        unsafe extern "C" fn(*mut LmEnv, *const LmVref, *mut *mut LmVref) -> LmStatus,
    pub env_remove: unsafe extern "C" fn(*mut LmEnv, *const LmVref) -> LmStatus,
    pub env_lookup: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut *mut LmVref) -> LmStatus,
    pub env_get: unsafe extern "C" fn(*const LmEnv, u32, *mut *mut LmVref) -> LmStatus,
    pub env_contains: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut bool) -> LmStatus,
    pub env_vtype_of: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut LmVtype) -> LmStatus,
    pub env_serialize: unsafe extern "C" fn(*const LmEnv, *mut LmBuffer) -> LmStatus,
    pub env_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmEnv) -> LmStatus,
    // //////// //
    // SOLUTION //
    // //////// //
    pub sol_free: unsafe extern "C" fn(*mut LmSol),
    pub sol_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmSol) -> LmStatus,
    // ////////// //
    // EXPRESSION //
    // ////////// //
    pub expr_free: unsafe extern "C" fn(*mut LmExpr),
    pub expr_default: extern "C" fn() -> *mut LmExpr,
    pub expr_new: unsafe extern "C" fn(*const LmEnv, *mut *mut LmExpr) -> LmStatus,
    pub expr_new_constant: unsafe extern "C" fn(*const LmEnv, f64, *mut *mut LmExpr) -> LmStatus,
    pub expr_clone: unsafe extern "C" fn(*const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub expr_deep_clone:
        unsafe extern "C" fn(*const LmExpr, *const LmEnv, *mut *mut LmExpr) -> LmStatus,
    pub expr_serialize: unsafe extern "C" fn(*const LmExpr, *mut LmBuffer) -> LmStatus,
    pub expr_deserialize:
        unsafe extern "C" fn(*const u8, usize, *const LmEnv, *mut *mut LmExpr) -> LmStatus,
    pub expr_env: unsafe extern "C" fn(*const LmExpr, *mut *mut LmEnv) -> LmStatus,
    pub expr_offset: unsafe extern "C" fn(*const LmExpr, *mut f64) -> LmStatus,
    pub expr_linear: unsafe extern "C" fn(*const LmExpr, *mut *mut LmLinearTerm) -> LmStatus,
    pub expr_quadratic: unsafe extern "C" fn(*const LmExpr, *mut *mut LmQuadraticTerm) -> LmStatus,
    pub expr_higher_order:
        unsafe extern "C" fn(*const LmExpr, *mut *mut LmHigherOrderTerm) -> LmStatus,
    pub expr_num_vars: unsafe extern "C" fn(*const LmExpr, *mut usize) -> LmStatus,
    pub expr_vtypes: unsafe extern "C" fn(*const LmExpr, *mut LmVtypeBuffer) -> LmStatus,
    pub expr_vars: unsafe extern "C" fn(*const LmExpr, *mut LmVrefBuffer) -> LmStatus,
    pub expr_complete_vars: unsafe extern "C" fn(*const LmExpr, *mut LmVrefBuffer) -> LmStatus,
    pub expr_degree: unsafe extern "C" fn(*const LmExpr, *mut usize) -> LmStatus,
    pub expr_is_constant: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    pub expr_has_quadratic: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    pub expr_has_higher_order: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    pub expr_eq: unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut bool) -> LmStatus,
    pub expr_equal_contents:
        unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut bool) -> LmStatus,
    // ////////// //
    // Constraint //
    // ////////// //
    pub constr_free: unsafe extern "C" fn(*mut LmConstraint),
    pub constr_new: unsafe extern "C" fn(
        *const LmExpr,
        f64,
        LmComparator,
        *const c_char,
        *mut *mut LmConstraint,
    ) -> LmStatus,
    pub constr_lhs: unsafe extern "C" fn(*const LmConstraint, *mut *mut LmExpr) -> LmStatus,
    pub constr_rhs: unsafe extern "C" fn(*const LmConstraint, *mut f64) -> LmStatus,
    pub constr_comparator: unsafe extern "C" fn(*const LmConstraint, *mut LmComparator) -> LmStatus,
    pub constr_name: unsafe extern "C" fn(*const LmConstraint, *mut LmBuffer) -> LmStatus,
    pub constr_set_name: unsafe extern "C" fn(*mut LmConstraint, *const c_char) -> LmStatus,
    // //////////////////// //
    // ConstraintCollection //
    // //////////////////// //
    pub cc_free: unsafe extern "C" fn(*mut LmCC),
    pub cc_default: extern "C" fn() -> *mut LmCC,
    pub cc_new: extern "C" fn() -> *mut LmCC,
    pub cc_clone: unsafe extern "C" fn(*const LmCC, *mut *mut LmCC) -> LmStatus,
    pub cc_deep_clone: unsafe extern "C" fn(*const LmCC, *const LmEnv, *mut *mut LmCC) -> LmStatus,
    pub cc_eq: unsafe extern "C" fn(*const LmCC, *const LmCC, *mut bool) -> LmStatus,
    pub cc_equal_contents: unsafe extern "C" fn(*const LmCC, *const LmCC, *mut bool) -> LmStatus,
    pub cc_vtypes: unsafe extern "C" fn(*const LmCC, *mut LmVtypeBuffer) -> LmStatus,
    pub cc_ctypes: unsafe extern "C" fn(*const LmCC, *mut LmCmpBuffer) -> LmStatus,
    pub cc_vars: unsafe extern "C" fn(*const LmCC, *mut LmVrefBuffer) -> LmStatus,
    pub cc_complete_vars: unsafe extern "C" fn(*const LmCC, *mut LmVrefBuffer) -> LmStatus,
    pub cc_is_empty: unsafe extern "C" fn(*const LmCC, *mut bool) -> LmStatus,
    pub cc_len: unsafe extern "C" fn(*const LmCC, *mut usize) -> LmStatus,
    pub cc_get:
        unsafe extern "C" fn(*const LmCC, *const c_char, *mut *mut LmConstraint) -> LmStatus,
    pub cc_add_constraint: unsafe extern "C" fn(
        *mut LmCC,
        *const LmConstraint,
        *const c_char,
        *mut LmBuffer,
    ) -> LmStatus,
    pub cc_add_collection: unsafe extern "C" fn(*mut LmCC, *const LmCC, *const c_char) -> LmStatus,
    pub cc_set_constraint:
        unsafe extern "C" fn(*mut LmCC, *const c_char, *const LmConstraint) -> LmStatus,
    pub cc_remove_constraint: unsafe extern "C" fn(*mut LmCC, *const c_char) -> LmStatus,
    pub cc_substitute: unsafe extern "C" fn(*mut LmCC, *const LmVref, *const LmExpr) -> LmStatus,
    pub cc_serialize: unsafe extern "C" fn(*const LmCC, *mut LmBuffer) -> LmStatus,
    pub cc_deserialize:
        unsafe extern "C" fn(*const u8, usize, *const LmEnv, *mut *mut LmCC) -> LmStatus,
    // ////// //
    // VarRef //
    // ////// //
    pub vref_free: unsafe extern "C" fn(*mut LmVref),
    pub vref_new: unsafe extern "C" fn(u32, *const LmEnv, *mut *mut LmVref) -> LmStatus,
    pub vref_id: unsafe extern "C" fn(*const LmVref, *mut u32) -> LmStatus,
    pub vref_check_living: unsafe extern "C" fn(*const LmVref, *mut bool) -> LmStatus,
    pub vref_name: unsafe extern "C" fn(*const LmVref, *mut LmBuffer) -> LmStatus,
    pub vref_vtype: unsafe extern "C" fn(*const LmVref, *mut LmVtype) -> LmStatus,
    pub vref_bounds: unsafe extern "C" fn(*const LmVref, *mut *mut LmBounds) -> LmStatus,
    pub vref_inverted: unsafe extern "C" fn(*const LmVref, *mut *mut LmVref) -> LmStatus,
    pub vref_hash: unsafe extern "C" fn(*const LmVref, *mut u64) -> LmStatus,
    pub vref_eq: unsafe extern "C" fn(*const LmVref, *const LmVref, *mut bool) -> LmStatus,
    pub vref_to_var: unsafe extern "C" fn(*const LmVref, *mut *mut LmVar) -> LmStatus,
    // //////// //
    // Variable //
    // //////// //
    pub var_free: unsafe extern "C" fn(*mut LmVar),
    pub var_name: unsafe extern "C" fn(*const LmVar, *mut LmBuffer) -> LmStatus,
    pub var_vtype: unsafe extern "C" fn(*const LmVar, *mut LmVtype) -> LmStatus,
    pub var_bounds: unsafe extern "C" fn(*const LmVar, *mut *mut LmBounds) -> LmStatus,
    pub var_inverted: unsafe extern "C" fn(*const LmVar, *mut u32) -> LmStatus,
    // ///// //
    // Specs //
    // ///// //
    pub specs_free: unsafe extern "C" fn(*mut LmSpecs),
    pub specs_new: unsafe extern "C" fn(
        *const LmSense,
        *const LmVtypeBuffer,
        *const LmCtypeBuffer,
        usize,
        usize,
        usize,
        *mut *mut LmSpecs,
    ) -> LmStatus,
    pub specs_sense: unsafe extern "C" fn(*const LmSpecs, *mut LmSense) -> LmStatus,
    pub specs_vtypes: unsafe extern "C" fn(*const LmSpecs, *mut LmVtypeBuffer) -> LmStatus,
    pub specs_ctypes: unsafe extern "C" fn(*const LmSpecs, *mut LmCtypeBuffer) -> LmStatus,
    pub specs_max_degree: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
    pub specs_max_constraint_degree: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
    pub specs_max_num_variables: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
    pub specs_satisfies:
        unsafe extern "C" fn(*const LmSpecs, *const LmSpecs, *mut bool) -> LmStatus,
    pub specs_clone: unsafe extern "C" fn(*const LmSpecs, *mut *mut LmSpecs) -> LmStatus,
}

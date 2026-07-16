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

/// Typed assignment value used by the C API.
///
/// C-ABI-stable mirror of LunaModel's Rust `Assignment` enum: `vtype` names the
/// variable kind and `value` holds the numeric assignment (widened to `f64`).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LmAssignment {
    pub vtype: LmVtype,
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
#[derive(Default)]
pub struct LmBuffer {
    /// Pointer to the first byte, or null for an empty buffer.
    pub data: *mut u8,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// An owned, heap-allocated string buffer the caller takes ownership of.
///
/// String buffers returned by LunaModel must be released with
/// [`lm_str_buffer_free`]. The fields mirror a `Vec<*mut c_char>` (each element
/// an owned NUL-terminated C string) so it can be rebuilt for deallocation.
#[repr(C)]
#[derive(Default)]
pub struct LmStrBuffer {
    /// Pointer to the first C string, or null for an empty buffer.
    pub data: *mut *mut c_char,
    /// Number of valid strings.
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
#[derive(Default)]
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
#[derive(Default)]
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
#[derive(Default)]
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
#[derive(Default)]
pub struct LmVrefBuffer {
    /// Pointer to the first VarRef, or null for an empty buffer.
    pub data: *mut *mut LmVref,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// Owned array of optional column handles.
///
/// Null entries represent names that were not present. Release the buffer with
/// [`lm_column_buffer_free`].
#[repr(C)]
#[derive(Default)]
pub struct LmColumnBuffer {
    pub data: *mut *mut LmColumn,
    pub len: usize,
    pub cap: usize,
}

/// An owned, heap-allocated u32 buffer the caller takes ownership of.
///
/// u32 buffers returned by LunaModel must be released
/// with [`lm_u32_buffer_free`]. The fields mirror a `Vec<u32>` so it can
/// be rebuilt for deallocation.
#[repr(C)]
#[derive(Default)]
pub struct LmU32Buffer {
    /// Pointer to the first u32, or null for an empty buffer.
    pub data: *mut u32,
    /// Number of valid u32s.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// An owned, heap-allocated `f64` buffer the caller takes ownership of.
///
/// f64 buffers returned by LunaModel must be released with
/// [`lm_f64_buffer_free`]. The fields mirror a `Vec<f64>` so it can be rebuilt
/// for deallocation.
#[repr(C)]
#[derive(Default)]
pub struct LmF64Buffer {
    /// Pointer to the first f64, or null for an empty buffer.
    pub data: *mut f64,
    /// Number of valid f64s.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// An owned, heap-allocated ResultView buffer the caller takes ownership of.
///
/// ResultView buffers returned by LunaModel must be released
/// with [`lm_result_view_buffer_free`]. The fields mirror a `Vec<ResultView>` so it can
/// be rebuilt for deallocation.
#[repr(C)]
#[derive(Default)]
pub struct LmResultViewBuffer {
    /// Pointer to the first ResultView, or null for an empty buffer.
    pub data: *mut *mut LmResultView,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// An owned, heap-allocated SampleView buffer the caller takes ownership of.
///
/// SampleView buffers returned by LunaModel must be released
/// with [`lm_sample_view_buffer_free`]. The fields mirror a `Vec<SampleView>` so it can
/// be rebuilt for deallocation.
#[repr(C)]
#[derive(Default)]
pub struct LmSampleViewBuffer {
    /// Pointer to the first SampleView, or null for an empty buffer.
    pub data: *mut *mut LmSampleView,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// A linear item.
#[repr(C)]
pub struct LmLinearItem {
    /// The variable index of the linear entry.
    pub idx: u32,
    /// The bias of the linear entry.
    pub bias: f64,
}

/// An owned, heap-allocated linear items buffer the caller takes ownership of.
///
/// Comparator buffers returned by LunaModel must be released
/// with [`lm_linear_item_buffer_free`]. The fields mirror a `Vec<LmLinearItem>` so it can
/// be rebuilt for deallocation.
#[repr(C)]
#[derive(Default)]
pub struct LmLinearItemBuffer {
    /// Pointer to the first LmLinearItem, or null for an empty buffer.
    pub data: *mut LmLinearItem,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// A quadratic item.
#[repr(C)]
pub struct LmQuadraticItem {
    /// The variable index of the one quadratic item entry.
    pub u_idx: u32,
    /// The variable index of the other quadratic item entry.
    pub v_idx: u32,
    /// The bias of the quadratic entry.
    pub bias: f64,
}

/// An owned, heap-allocated quadratic items buffer the caller takes ownership of.
///
/// Comparator buffers returned by LunaModel must be released
/// with [`lm_quadratic_item_buffer_free`]. The fields mirror a `Vec<LmQuadraticItem>` so it can
/// be rebuilt for deallocation.
#[repr(C)]
#[derive(Default)]
pub struct LmQuadraticItemBuffer {
    /// Pointer to the first LmQuadraticItem, or null for an empty buffer.
    pub data: *mut LmQuadraticItem,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// A higher-order item.
#[repr(C)]
pub struct LmHigherOrderItem {
    /// An owned, heap-allocated variable index buffer the caller takes ownership of.
    pub indices: LmU32Buffer,
    /// The bias of the quadratic entry.
    pub bias: f64,
}

/// An owned, heap-allocated higher-order items buffer the caller takes ownership of.
///
/// Comparator buffers returned by LunaModel must be released
/// with [`lm_higher_order_item_buffer_free`]. The fields mirror a `Vec<LmHigherOrderItem>` so it can
/// be rebuilt for deallocation.
#[repr(C)]
pub struct LmHigherOrderItemBuffer {
    /// Pointer to the first LmHigherOrderItem, or null for an empty buffer.
    pub data: *mut LmHigherOrderItem,
    /// Number of valid bytes.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// One `(variable name, column)` entry of a solution's sample map.
#[repr(C)]
pub struct LmIndexMapItem {
    /// Position of the entry in the underlying `IndexMap` (column order).
    ///
    /// Stable for a given solution and usable with index-based accessors such as
    /// [`lm_sol_assignment_idx`].
    pub index: usize,
    /// Owned, NUL-terminated variable name.
    pub key: *mut c_char,
    /// Owned column handle for the variable.
    pub value: *mut LmColumn,
}

/// An owned, heap-allocated index-map-item buffer the caller takes ownership of.
///
/// Returned by [`lm_sol_samples`] and released with [`lm_index_map_buffer_free`],
/// which also frees each entry's owned key string and column handle. The fields
/// mirror a `Vec<LmIndexMapItem>` so it can be rebuilt for deallocation.
#[repr(C)]
pub struct LmIndexMapItemBuffer {
    /// Pointer to the first LmIndexMapItem, or null for an empty buffer.
    pub data: *mut LmIndexMapItem,
    /// Number of valid entries.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// One `(name, boolean series)` entry of a per-name flag map.
///
/// Used for solution constraint and variable-bound feasibility flags. The `data`
/// array holds one flag per sample row.
#[repr(C)]
pub struct LmBoolMapEntry {
    /// Owned, NUL-terminated name (constraint or variable).
    pub key: *mut c_char,
    /// Pointer to the first flag, or null for an empty series.
    pub data: *mut bool,
    /// Number of flags.
    pub len: usize,
    /// Allocated capacity (needed to reconstruct the `Vec` when freeing).
    pub cap: usize,
}

/// An owned, heap-allocated bool-map buffer the caller takes ownership of.
///
/// Returned by [`lm_sol_constraints`] / [`lm_sol_variable_bounds`] and released
/// with [`lm_bool_map_buffer_free`], which also frees each entry's owned key
/// string and flag array. The fields mirror a `Vec<LmBoolMapEntry>`.
#[repr(C)]
pub struct LmBoolMapBuffer {
    /// Pointer to the first LmBoolMapEntry, or null for an empty buffer.
    pub data: *mut LmBoolMapEntry,
    /// Number of valid entries.
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

/// Opaque handle to a solution column (all values of one variable).
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmColumn {
    _private: [u8; 0],
}

/// Opaque handle to a borrowed LunaModel result-view.
///
/// Created by [`lm_sol_result`] and released with [`lm_result_view_free`]. It
/// borrows the solution it was created from: the originating `LmSol` must
/// outlive the view and must not be mutated while the view is alive.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmResultView {
    _private: [u8; 0],
}

/// Opaque handle to a borrowed LunaModel sample-view.
///
/// Created by [`lm_sol_sample`] and released with [`lm_sample_view_free`]. It
/// borrows the solution it was created from: the originating `LmSol` must
/// outlive the view and must not be mutated while the view is alive.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmSampleView {
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
pub struct LmLinear {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel quadratic terms reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmQuadratic {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel higher-order terms reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmHigherOrder {
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
    pub str_buffer_free: unsafe extern "C" fn(LmStrBuffer),
    pub vtype_buffer_free: unsafe extern "C" fn(LmVtypeBuffer),
    pub ctype_buffer_free: unsafe extern "C" fn(LmCtypeBuffer),
    pub cmp_buffer_free: unsafe extern "C" fn(LmCmpBuffer),
    pub vref_buffer_free: unsafe extern "C" fn(LmVrefBuffer),
    pub column_buffer_free: unsafe extern "C" fn(LmColumnBuffer),
    pub u32_buffer_free: unsafe extern "C" fn(LmU32Buffer),
    pub f64_buffer_free: unsafe extern "C" fn(LmF64Buffer),
    pub linear_item_buffer_free: unsafe extern "C" fn(LmLinearItemBuffer),
    pub quadratic_item_buffer_free: unsafe extern "C" fn(LmQuadraticItemBuffer),
    pub higher_order_item_buffer_free: unsafe extern "C" fn(LmHigherOrderItemBuffer),
    pub result_view_buffer_free: unsafe extern "C" fn(LmResultViewBuffer),
    pub sample_view_buffer_free: unsafe extern "C" fn(LmSampleViewBuffer),
    pub index_map_buffer_free: unsafe extern "C" fn(LmIndexMapItemBuffer),
    pub bool_map_buffer_free: unsafe extern "C" fn(LmBoolMapBuffer),
    // ///// //
    // MODEL //
    // ///// //
    pub model_free: unsafe extern "C" fn(*mut LmModel),
    // Model -- creation
    pub model_default: extern "C" fn() -> *mut LmModel,
    pub model_new:
        unsafe extern "C" fn(*const c_char, *const LmSense, *mut *mut LmModel) -> LmStatus,
    pub model_with_env: unsafe extern "C" fn(
        *const c_char,
        *const LmSense,
        *const LmEnv,
        *mut *mut LmModel,
    ) -> LmStatus,
    // Model -- general
    pub model_clone: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    pub model_retain: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    pub model_deep_clone: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    pub model_satisfies:
        unsafe extern "C" fn(*const LmModel, *const LmSpecs, *mut bool) -> LmStatus,
    pub model_add_var: unsafe extern "C" fn(
        *mut LmModel,
        *const c_char,
        LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub model_add_var_with_fallback: unsafe extern "C" fn(
        *mut LmModel,
        *const c_char,
        LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub model_eq: unsafe extern "C" fn(*const LmModel, *const LmModel, *mut bool) -> LmStatus,
    pub model_equal_contents:
        unsafe extern "C" fn(*const LmModel, *const LmModel, *mut bool) -> LmStatus,
    pub model_evaluate_solution:
        unsafe extern "C" fn(*const LmModel, *const LmSol, *mut *mut LmSol) -> LmStatus,
    pub model_evaluate_solution_with_tol:
        unsafe extern "C" fn(*const LmModel, *const LmSol, *const f64, *mut *mut LmSol) -> LmStatus,
    pub model_evaluate_sample: unsafe extern "C" fn(
        *const LmModel,
        *const LmSampleView,
        *const f64,
        *mut *mut LmResultView,
    ) -> LmStatus,
    pub model_violated_constraints: unsafe extern "C" fn(
        *const LmModel,
        *const LmSampleView,
        *const f64,
        *mut *mut LmCC,
    ) -> LmStatus,
    pub model_substitute:
        unsafe extern "C" fn(*mut LmModel, *const LmVref, *const LmExpr) -> LmStatus,
    // Model -- access
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
    pub model_constraints: unsafe extern "C" fn(*const LmModel, *mut *mut LmCC) -> LmStatus,
    // Model -- setter
    pub model_set_name: unsafe extern "C" fn(*mut LmModel, *const c_char) -> LmStatus,
    pub model_set_sense: unsafe extern "C" fn(*mut LmModel, LmSense) -> LmStatus,
    pub model_set_objective:
        unsafe extern "C" fn(*mut LmModel, *const LmExpr, *const LmSense) -> LmStatus,
    pub model_set_constraints: unsafe extern "C" fn(*mut LmModel, *const LmCC) -> LmStatus,
    // Model -- serializer
    pub model_serialize: unsafe extern "C" fn(*const LmModel, *mut LmBuffer) -> LmStatus,
    pub model_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmModel) -> LmStatus,
    // /////////// //
    // ENVIRONMENT //
    // /////////// //
    pub env_free: unsafe extern "C" fn(*mut LmEnv),
    pub env_default: extern "C" fn() -> *mut LmEnv,
    pub env_new: unsafe extern "C" fn(
        *const *const LmVar,
        *const u32,
        usize,
        *const *const c_char,
        *const u32,
        usize,
        u32,
        *mut *mut LmEnv,
    ) -> LmStatus,
    // Env -- general
    pub env_clone: unsafe extern "C" fn(*const LmEnv, *mut *mut LmEnv) -> LmStatus,
    pub env_deep_clone: unsafe extern "C" fn(*const LmEnv, *mut *mut LmEnv) -> LmStatus,
    pub env_eq: unsafe extern "C" fn(*const LmEnv, *const LmEnv, *mut bool) -> LmStatus,
    pub env_equal_contents: unsafe extern "C" fn(*const LmEnv, *const LmEnv, *mut bool) -> LmStatus,
    // Env -- access
    pub env_len: unsafe extern "C" fn(*const LmEnv, *mut usize) -> LmStatus,
    pub env_is_empty: unsafe extern "C" fn(*const LmEnv, *mut bool) -> LmStatus,
    pub env_vars: unsafe extern "C" fn(*const LmEnv, *mut LmVrefBuffer) -> LmStatus,
    pub env_id: unsafe extern "C" fn(*const LmEnv, *mut LmEnvId) -> LmStatus,
    pub env_lookup: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut *mut LmVref) -> LmStatus,
    pub env_get: unsafe extern "C" fn(*const LmEnv, u32, *mut *mut LmVref) -> LmStatus,
    pub env_contains: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut bool) -> LmStatus,
    pub env_vtype_of: unsafe extern "C" fn(*const LmEnv, *const c_char, *mut LmVtype) -> LmStatus,
    pub env_sort: unsafe extern "C" fn(
        *const LmEnv,
        *const *const c_char,
        usize,
        *mut LmStrBuffer,
    ) -> LmStatus,
    pub env_next_idx: unsafe extern "C" fn(*const LmEnv, *mut u32) -> LmStatus,
    // Env -- modification
    pub env_insert: unsafe extern "C" fn(
        *mut LmEnv,
        *const c_char,
        LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub env_insert_with_fallback: unsafe extern "C" fn(
        *mut LmEnv,
        *const c_char,
        LmVtype,
        *const LmLazyBounds,
        *const u64,
        usize,
        *mut *mut LmVref,
    ) -> LmStatus,
    pub env_insert_inverted:
        unsafe extern "C" fn(*mut LmEnv, *const LmVref, *mut *mut LmVref) -> LmStatus,
    pub env_remove: unsafe extern "C" fn(*mut LmEnv, *const LmVref) -> LmStatus,
    pub env_set_inverted: unsafe extern "C" fn(*mut LmEnv, u32, bool, u32) -> LmStatus,
    // Env -- serializer
    pub env_serialize: unsafe extern "C" fn(*const LmEnv, *mut LmBuffer) -> LmStatus,
    pub env_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmEnv) -> LmStatus,
    // ////// //
    // COLUMN //
    // ////// //
    pub col_free: unsafe extern "C" fn(*mut LmColumn),
    // Column -- creation
    pub col_new: unsafe extern "C" fn(LmVtype, *const f64, usize, *mut *mut LmColumn) -> LmStatus,
    pub col_with: unsafe extern "C" fn(LmAssignment, *mut *mut LmColumn) -> LmStatus,
    // Column -- general
    pub col_clone: unsafe extern "C" fn(*const LmColumn, *mut *mut LmColumn) -> LmStatus,
    pub col_eq: unsafe extern "C" fn(*const LmColumn, *const LmColumn, *mut bool) -> LmStatus,
    pub col_filter_by_mask:
        unsafe extern "C" fn(*const LmColumn, *const bool, usize, *mut *mut LmColumn) -> LmStatus,
    pub col_extract: unsafe extern "C" fn(*const LmColumn, usize, *mut *mut LmColumn) -> LmStatus,
    // Column -- access
    pub col_len: unsafe extern "C" fn(*const LmColumn, *mut usize) -> LmStatus,
    pub col_is_empty: unsafe extern "C" fn(*const LmColumn, *mut bool) -> LmStatus,
    pub col_vtype: unsafe extern "C" fn(*const LmColumn, *mut LmVtype) -> LmStatus,
    pub col_assignment: unsafe extern "C" fn(*const LmColumn, usize, *mut LmAssignment) -> LmStatus,
    pub col_values: unsafe extern "C" fn(*const LmColumn, *mut *const f64, *mut usize) -> LmStatus,
    // Column -- modification
    pub col_push: unsafe extern "C" fn(*mut LmColumn, LmAssignment) -> LmStatus,
    pub col_try_push: unsafe extern "C" fn(*mut LmColumn, f64, *const f64) -> LmStatus,
    pub col_remove: unsafe extern "C" fn(*mut LmColumn, usize) -> LmStatus,
    // /////////// //
    // RESULT_VIEW //
    // /////////// //
    pub result_view_free: unsafe extern "C" fn(*mut LmResultView),
    pub result_view_counts: unsafe extern "C" fn(*const LmResultView, *mut usize) -> LmStatus,
    pub result_view_obj_value: unsafe extern "C" fn(*const LmResultView, *mut f64) -> LmStatus,
    pub result_view_raw_energy: unsafe extern "C" fn(*const LmResultView, *mut f64) -> LmStatus,
    pub result_view_constraints:
        unsafe extern "C" fn(*const LmResultView, *mut LmBoolMapBuffer) -> LmStatus,
    pub result_view_variable_bounds:
        unsafe extern "C" fn(*const LmResultView, *mut LmBoolMapBuffer) -> LmStatus,
    pub result_view_feasible: unsafe extern "C" fn(*const LmResultView, *mut bool) -> LmStatus,
    pub result_view_sample:
        unsafe extern "C" fn(*const LmResultView, *mut *mut LmSampleView) -> LmStatus,
    // /////////// //
    // SAMPLE_VIEW //
    // /////////// //
    pub sample_view_free: unsafe extern "C" fn(*mut LmSampleView),
    pub sample_view_index: unsafe extern "C" fn(*const LmSampleView, *mut usize) -> LmStatus,
    pub sample_view_variable_names:
        unsafe extern "C" fn(*const LmSampleView, *mut LmStrBuffer) -> LmStatus,
    pub sample_view_assignment:
        unsafe extern "C" fn(*const LmSampleView, *const c_char, *mut LmAssignment) -> LmStatus,
    pub sample_view_assignment_idx:
        unsafe extern "C" fn(*const LmSampleView, usize, *mut LmAssignment) -> LmStatus,
    // //////// //
    // SOLUTION //
    // //////// //
    pub sol_free: unsafe extern "C" fn(*mut LmSol),
    pub sol_default: extern "C" fn() -> *mut LmSol,
    pub sol_with_sense: extern "C" fn(LmSense) -> *mut LmSol,
    pub sol_with_capacity: extern "C" fn(usize) -> *mut LmSol,
    pub sol_from_env: unsafe extern "C" fn(*const LmEnv, *mut *mut LmSol) -> LmStatus,
    pub sol_from_random: unsafe extern "C" fn(
        usize,
        *const u64,
        *const LmEnv,
        *const LmModel,
        *const LmSense,
        *const f64,
        *mut *mut LmSol,
    ) -> LmStatus,
    // Sol -- general
    pub sol_clone: unsafe extern "C" fn(*const LmSol, *mut *mut LmSol) -> LmStatus,
    pub sol_eq: unsafe extern "C" fn(*const LmSol, *const LmSol, *mut bool) -> LmStatus,
    pub sol_equal_contents: unsafe extern "C" fn(*const LmSol, *const LmSol, *mut bool) -> LmStatus,
    pub sol_aggregate: unsafe extern "C" fn(*mut LmSol) -> LmStatus,
    pub sol_merge_many: unsafe extern "C" fn(
        *const *const LmSol,
        usize,
        *const LmModel,
        *const f64,
        *mut *mut LmSol,
    ) -> LmStatus,
    pub sol_filter_by_mask:
        unsafe extern "C" fn(*const LmSol, *const bool, usize, *mut *mut LmSol) -> LmStatus,
    // Sol -- properties
    pub sol_samples: unsafe extern "C" fn(*const LmSol, *mut LmIndexMapItemBuffer) -> LmStatus,
    pub sol_column:
        unsafe extern "C" fn(*const LmSol, *const c_char, *mut *mut LmColumn) -> LmStatus,
    pub sol_counts: unsafe extern "C" fn(*const LmSol, *mut *const usize, *mut usize) -> LmStatus,
    pub sol_raw_energies:
        unsafe extern "C" fn(*const LmSol, *mut *const f64, *mut usize) -> LmStatus,
    pub sol_obj_values: unsafe extern "C" fn(*const LmSol, *mut *const f64, *mut usize) -> LmStatus,
    pub sol_feasible: unsafe extern "C" fn(*const LmSol, *mut *const bool, *mut usize) -> LmStatus,
    pub sol_constraints: unsafe extern "C" fn(*const LmSol, *mut LmBoolMapBuffer) -> LmStatus,
    pub sol_variable_bounds: unsafe extern "C" fn(*const LmSol, *mut LmBoolMapBuffer) -> LmStatus,
    pub sol_timing: unsafe extern "C" fn(*const LmSol, *mut *mut LmTiming) -> LmStatus,
    pub sol_sense: unsafe extern "C" fn(*const LmSol, *mut LmSense) -> LmStatus,
    // Sol -- access
    pub sol_n_samples: unsafe extern "C" fn(*const LmSol, *mut usize) -> LmStatus,
    pub sol_len: unsafe extern "C" fn(*const LmSol, *mut usize) -> LmStatus,
    pub sol_variable_names: unsafe extern "C" fn(*const LmSol, *mut LmStrBuffer) -> LmStatus,
    pub sol_is_empty: unsafe extern "C" fn(*const LmSol, *mut bool) -> LmStatus,
    pub sol_sample_len: unsafe extern "C" fn(*const LmSol, *mut usize) -> LmStatus,
    pub sol_value: unsafe extern "C" fn(*const LmSol, usize, *const c_char, *mut f64) -> LmStatus,
    pub sol_assignment:
        unsafe extern "C" fn(*const LmSol, usize, *const c_char, *mut LmAssignment) -> LmStatus,
    pub sol_assignment_idx:
        unsafe extern "C" fn(*const LmSol, usize, usize, *mut LmAssignment) -> LmStatus,
    pub sol_result: unsafe extern "C" fn(*const LmSol, usize, *mut *mut LmResultView) -> LmStatus,
    pub sol_results: unsafe extern "C" fn(*const LmSol, *mut LmResultViewBuffer) -> LmStatus,
    pub sol_sample_view:
        unsafe extern "C" fn(*const LmSol, usize, *mut *mut LmSampleView) -> LmStatus,
    pub sol_sample_views: unsafe extern "C" fn(*const LmSol, *mut LmSampleViewBuffer) -> LmStatus,
    pub sol_best: unsafe extern "C" fn(*const LmSol, *mut LmResultViewBuffer) -> LmStatus,
    pub sol_extract: unsafe extern "C" fn(*const LmSol, usize, *mut *mut LmSol) -> LmStatus,
    // Sol -- modification
    pub sol_push: unsafe extern "C" fn(
        *mut LmSol,
        *const *const c_char,
        *const LmAssignment,
        usize,
        usize,
        *const f64,
    ) -> LmStatus,
    pub sol_add_empty_binary: unsafe extern "C" fn(*mut LmSol, *const c_char) -> LmStatus,
    pub sol_add_empty_spin: unsafe extern "C" fn(*mut LmSol, *const c_char) -> LmStatus,
    pub sol_add_empty_integer: unsafe extern "C" fn(*mut LmSol, *const c_char) -> LmStatus,
    pub sol_add_empty_real: unsafe extern "C" fn(*mut LmSol, *const c_char) -> LmStatus,
    pub sol_add_binary:
        unsafe extern "C" fn(*mut LmSol, *const c_char, *const f64, usize, *const f64) -> LmStatus,
    pub sol_add_spin:
        unsafe extern "C" fn(*mut LmSol, *const c_char, *const f64, usize, *const f64) -> LmStatus,
    pub sol_add_integer:
        unsafe extern "C" fn(*mut LmSol, *const c_char, *const f64, usize, *const f64) -> LmStatus,
    pub sol_add_real:
        unsafe extern "C" fn(*mut LmSol, *const c_char, *const f64, usize) -> LmStatus,
    pub sol_add_col: unsafe extern "C" fn(
        *mut LmSol,
        LmVtype,
        *const c_char,
        *const f64,
        usize,
        *const f64,
    ) -> LmStatus,
    pub sol_remove_col:
        unsafe extern "C" fn(*mut LmSol, *const c_char, *mut *mut LmColumn) -> LmStatus,
    pub sol_remove_cols: unsafe extern "C" fn(
        *mut LmSol,
        *const *const c_char,
        usize,
        *mut LmColumnBuffer,
    ) -> LmStatus,
    pub sol_set_counts: unsafe extern "C" fn(*mut LmSol, *const usize, usize) -> LmStatus,
    pub sol_set_timing: unsafe extern "C" fn(*mut LmSol, *const LmTiming) -> LmStatus,
    pub sol_set_sense: unsafe extern "C" fn(*mut LmSol, LmSense) -> LmStatus,
    // Sol -- convenience
    pub sol_cvar:
        unsafe extern "C" fn(*const LmSol, f64, *const LmValueSource, *mut f64) -> LmStatus,
    pub sol_expectation_value:
        unsafe extern "C" fn(*const LmSol, *const LmValueSource, *mut f64) -> LmStatus,
    pub sol_temperature_weighted:
        unsafe extern "C" fn(*const LmSol, f64, *const LmValueSource, *mut f64) -> LmStatus,
    pub sol_feasibility_ratio: unsafe extern "C" fn(*const LmSol, *mut f64) -> LmStatus,
    pub sol_highest_constraint_violations:
        unsafe extern "C" fn(*const LmSol, *mut LmBuffer) -> LmStatus,
    // Sol -- serializer
    pub sol_serialize: unsafe extern "C" fn(*const LmSol, *mut LmBuffer) -> LmStatus,
    pub sol_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmSol) -> LmStatus,
    // ////////// //
    // EXPRESSION //
    // ////////// //
    pub expr_free: unsafe extern "C" fn(*mut LmExpr),
    pub expr_default: extern "C" fn() -> *mut LmExpr,
    pub expr_new: unsafe extern "C" fn(*const LmEnv, *mut *mut LmExpr) -> LmStatus,
    pub expr_new_constant: unsafe extern "C" fn(*const LmEnv, f64, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_double: unsafe extern "C" fn(f64, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_size_t: unsafe extern "C" fn(usize, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_linear: unsafe extern "C" fn(*const LmLinear, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_quadratic: unsafe extern "C" fn(*const LmQuadratic, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_higher_order:
        unsafe extern "C" fn(*const LmHigherOrder, *mut *mut LmExpr) -> LmStatus,
    pub expr_from_dense_quadratic: unsafe extern "C" fn(
        *const f64,
        usize,
        usize,
        *const f64,
        *const LmEnv,
        *mut *mut LmExpr,
    ) -> LmStatus,
    pub expr_clone: unsafe extern "C" fn(*const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub expr_deep_clone:
        unsafe extern "C" fn(*const LmExpr, *const LmEnv, *mut *mut LmExpr) -> LmStatus,
    pub expr_deep_clone_many:
        unsafe extern "C" fn(*const *const LmExpr, usize, *mut *mut LmExpr) -> LmStatus,
    pub expr_evaluate_sample_quick:
        unsafe extern "C" fn(*const LmExpr, *const f64, usize, *mut f64) -> LmStatus,
    pub expr_evaluate_sample: unsafe extern "C" fn(
        *const LmExpr,
        *const *const c_char,
        *const f64,
        usize,
        *mut f64,
    ) -> LmStatus,
    pub expr_evaluate:
        unsafe extern "C" fn(*const LmExpr, *const LmSol, *mut LmF64Buffer) -> LmStatus,
    pub expr_serialize: unsafe extern "C" fn(*const LmExpr, *mut LmBuffer) -> LmStatus,
    pub expr_deserialize:
        unsafe extern "C" fn(*const u8, usize, *const LmEnv, *mut *mut LmExpr) -> LmStatus,
    pub expr_eq: unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut bool) -> LmStatus,
    pub expr_equal_contents:
        unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut bool) -> LmStatus,
    pub expr_env: unsafe extern "C" fn(*const LmExpr, *mut *mut LmEnv) -> LmStatus,
    pub expr_offset: unsafe extern "C" fn(*const LmExpr, *mut f64) -> LmStatus,
    pub expr_linear: unsafe extern "C" fn(*const LmExpr, *mut *mut LmLinear) -> LmStatus,
    pub expr_quadratic: unsafe extern "C" fn(*const LmExpr, *mut *mut LmQuadratic) -> LmStatus,
    pub expr_higher_order: unsafe extern "C" fn(*const LmExpr, *mut *mut LmHigherOrder) -> LmStatus,
    pub expr_num_vars: unsafe extern "C" fn(*const LmExpr, *mut usize) -> LmStatus,
    pub expr_vtypes: unsafe extern "C" fn(*const LmExpr, *mut LmVtypeBuffer) -> LmStatus,
    pub expr_vars: unsafe extern "C" fn(*const LmExpr, *mut LmVrefBuffer) -> LmStatus,
    pub expr_complete_vars: unsafe extern "C" fn(*const LmExpr, *mut LmVrefBuffer) -> LmStatus,
    pub expr_contains: unsafe extern "C" fn(*const LmExpr, *const LmVref, *mut bool) -> LmStatus,
    pub expr_degree: unsafe extern "C" fn(*const LmExpr, *mut usize) -> LmStatus,
    pub expr_is_constant: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    pub expr_has_quadratic: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    pub expr_has_higher_order: unsafe extern "C" fn(*const LmExpr, *mut bool) -> LmStatus,
    pub expr_set_offset: unsafe extern "C" fn(*mut LmExpr, f64) -> LmStatus,
    pub expr_add_linear_term: unsafe extern "C" fn(*mut LmExpr, u32, f64) -> LmStatus,
    pub expr_add_quadratic_term: unsafe extern "C" fn(*mut LmExpr, u32, u32, f64) -> LmStatus,
    pub expr_add_higher_order_term:
        unsafe extern "C" fn(*mut LmExpr, *const u32, usize, f64) -> LmStatus,
    pub expr_add_bias: unsafe extern "C" fn(*const LmExpr, f64, *mut *mut LmExpr) -> LmStatus,
    pub expr_add_vref:
        unsafe extern "C" fn(*const LmExpr, *const LmVref, *mut *mut LmExpr) -> LmStatus,
    pub expr_add_expr:
        unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub expr_sub_bias: unsafe extern "C" fn(*const LmExpr, f64, *mut *mut LmExpr) -> LmStatus,
    pub expr_rsub_bias: unsafe extern "C" fn(*const LmExpr, f64, *mut *mut LmExpr) -> LmStatus, // todo
    pub expr_sub_vref:
        unsafe extern "C" fn(*const LmExpr, *const LmVref, *mut *mut LmExpr) -> LmStatus,
    pub expr_sub_expr:
        unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub expr_mul_bias: unsafe extern "C" fn(*const LmExpr, f64, *mut *mut LmExpr) -> LmStatus,
    pub expr_mul_vref:
        unsafe extern "C" fn(*const LmExpr, *const LmVref, *mut *mut LmExpr) -> LmStatus,
    pub expr_mul_expr:
        unsafe extern "C" fn(*const LmExpr, *const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub expr_iadd_bias: unsafe extern "C" fn(*mut LmExpr, f64) -> LmStatus,
    pub expr_iadd_vref: unsafe extern "C" fn(*mut LmExpr, *const LmVref) -> LmStatus,
    pub expr_iadd_expr: unsafe extern "C" fn(*mut LmExpr, *const LmExpr) -> LmStatus,
    pub expr_isub_bias: unsafe extern "C" fn(*mut LmExpr, f64) -> LmStatus,
    pub expr_isub_vref: unsafe extern "C" fn(*mut LmExpr, *const LmVref) -> LmStatus,
    pub expr_isub_expr: unsafe extern "C" fn(*mut LmExpr, *const LmExpr) -> LmStatus,
    pub expr_imul_bias: unsafe extern "C" fn(*mut LmExpr, f64) -> LmStatus,
    pub expr_imul_vref: unsafe extern "C" fn(*mut LmExpr, *const LmVref) -> LmStatus,
    pub expr_imul_expr: unsafe extern "C" fn(*mut LmExpr, *const LmExpr) -> LmStatus,
    pub expr_neg: unsafe extern "C" fn(*const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub expr_pow: unsafe extern "C" fn(*const LmExpr, usize, *mut *mut LmExpr) -> LmStatus,
    pub expr_ipow: unsafe extern "C" fn(*mut LmExpr, usize) -> LmStatus,
    pub expr_separate: unsafe extern "C" fn(
        *const LmExpr,
        *const *const LmVref,
        usize,
        *mut *mut LmExpr,
        *mut *mut LmExpr,
    ) -> LmStatus,
    pub expr_substitute: unsafe extern "C" fn(
        *const LmExpr,
        *const LmVref,
        *const LmExpr,
        *mut *mut LmExpr,
    ) -> LmStatus,
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
    pub constr_clone: unsafe extern "C" fn(*const LmConstraint, *mut *mut LmConstraint) -> LmStatus,
    pub constr_deep_clone:
        unsafe extern "C" fn(*const LmConstraint, *const LmEnv, *mut *mut LmConstraint) -> LmStatus,
    pub constr_substitute:
        unsafe extern "C" fn(*mut LmConstraint, *const LmVref, *const LmExpr) -> LmStatus,
    pub constr_equal_contents:
        unsafe extern "C" fn(*const LmConstraint, *const LmConstraint, *mut bool) -> LmStatus,
    pub constr_eq:
        unsafe extern "C" fn(*const LmConstraint, *const LmConstraint, *mut bool) -> LmStatus,
    pub constr_evaluate_sample: unsafe extern "C" fn(
        *const LmConstraint,
        *const *const c_char,
        *const f64,
        usize,
        *const f64,
        *mut bool,
    ) -> LmStatus,
    pub constr_evaluate_sample_quick: unsafe extern "C" fn(
        *const LmConstraint,
        *const f64,
        usize,
        *const f64,
        *mut bool,
    ) -> LmStatus,
    pub constr_lhs: unsafe extern "C" fn(*const LmConstraint, *mut *mut LmExpr) -> LmStatus,
    pub constr_rhs: unsafe extern "C" fn(*const LmConstraint, *mut f64) -> LmStatus,
    pub constr_comparator: unsafe extern "C" fn(*const LmConstraint, *mut LmComparator) -> LmStatus,
    pub constr_name: unsafe extern "C" fn(*const LmConstraint, *mut LmBuffer) -> LmStatus,
    pub constr_auto_name: unsafe extern "C" fn(*const LmConstraint, *mut bool) -> LmStatus,
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
    pub cc_evaluate_sample: unsafe extern "C" fn(
        *const LmCC,
        *const *const c_char,
        *const f64,
        usize,
        *const f64,
        *mut LmBoolMapBuffer,
    ) -> LmStatus,
    pub cc_evaluate_sample_quick: unsafe extern "C" fn(
        *const LmCC,
        *const f64,
        usize,
        *const f64,
        *mut LmBoolMapBuffer,
    ) -> LmStatus,
    pub cc_vtypes: unsafe extern "C" fn(*const LmCC, *mut LmVtypeBuffer) -> LmStatus,
    pub cc_ctypes: unsafe extern "C" fn(*const LmCC, *mut LmCmpBuffer) -> LmStatus,
    pub cc_vars: unsafe extern "C" fn(*const LmCC, *mut LmVrefBuffer) -> LmStatus,
    pub cc_complete_vars: unsafe extern "C" fn(*const LmCC, *mut LmVrefBuffer) -> LmStatus,
    pub cc_is_empty: unsafe extern "C" fn(*const LmCC, *mut bool) -> LmStatus,
    pub cc_len: unsafe extern "C" fn(*const LmCC, *mut usize) -> LmStatus,
    pub cc_names: unsafe extern "C" fn(*const LmCC, *mut LmStrBuffer) -> LmStatus,
    pub cc_contains: unsafe extern "C" fn(*const LmCC, *const c_char, *mut bool) -> LmStatus,
    pub cc_get:
        unsafe extern "C" fn(*const LmCC, *const c_char, *mut *mut LmConstraint) -> LmStatus,
    pub cc_add_constraint: unsafe extern "C" fn(
        *mut LmCC,
        *const LmConstraint,
        *const c_char,
        *mut LmBuffer,
    ) -> LmStatus,
    pub cc_add_collection:
        unsafe extern "C" fn(*mut LmCC, *const LmCC, *const c_char, *mut LmStrBuffer) -> LmStatus,
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
    pub vref_clone: unsafe extern "C" fn(*const LmVref, *mut *mut LmVref) -> LmStatus,
    pub vref_id: unsafe extern "C" fn(*const LmVref, *mut u32) -> LmStatus,
    pub vref_check_living: unsafe extern "C" fn(*const LmVref, *mut bool) -> LmStatus,
    pub vref_name: unsafe extern "C" fn(*const LmVref, *mut LmBuffer) -> LmStatus,
    pub vref_vtype: unsafe extern "C" fn(*const LmVref, *mut LmVtype) -> LmStatus,
    pub vref_bounds: unsafe extern "C" fn(*const LmVref, *mut *mut LmBounds) -> LmStatus,
    pub vref_inverted: unsafe extern "C" fn(*const LmVref, *mut *mut LmVref) -> LmStatus,
    pub vref_hash: unsafe extern "C" fn(*const LmVref, *mut u64) -> LmStatus,
    pub vref_eq: unsafe extern "C" fn(*const LmVref, *const LmVref, *mut bool) -> LmStatus,
    pub vref_to_var: unsafe extern "C" fn(*const LmVref, *mut *mut LmVar) -> LmStatus,
    pub vref_mul_vref:
        unsafe extern "C" fn(*const LmVref, *const LmVref, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_mul_bias: unsafe extern "C" fn(*const LmVref, f64, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_add_vref:
        unsafe extern "C" fn(*const LmVref, *const LmVref, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_add_bias: unsafe extern "C" fn(*const LmVref, f64, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_sub_vref:
        unsafe extern "C" fn(*const LmVref, *const LmVref, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_sub_expr:
        unsafe extern "C" fn(*const LmVref, *const LmExpr, *mut *mut LmExpr) -> LmStatus,
    pub vref_sub_bias: unsafe extern "C" fn(*const LmVref, f64, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_rsub_bias: unsafe extern "C" fn(*const LmVref, f64, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_neg: unsafe extern "C" fn(*const LmVref, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_not: unsafe extern "C" fn(*const LmVref, out: *mut *mut LmVref) -> LmStatus,
    pub vref_pow: unsafe extern "C" fn(*const LmVref, usize, out: *mut *mut LmExpr) -> LmStatus,
    pub vref_eval: unsafe extern "C" fn(*const LmVref, f64, *const f64, out: *mut bool) -> LmStatus,
    // //////// //
    // Variable //
    // //////// //
    pub var_free: unsafe extern "C" fn(*mut LmVar),
    pub var_new: unsafe extern "C" fn(
        *const c_char,
        LmVtype,
        *const LmLazyBounds,
        *mut *mut LmVar,
    ) -> LmStatus,
    pub var_clone: unsafe extern "C" fn(*const LmVar, *mut *mut LmVar) -> LmStatus,
    pub var_eq: unsafe extern "C" fn(*const LmVar, *const LmVar, *mut bool) -> LmStatus,
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
        *const LmVtype,
        usize,
        *const LmCtype,
        usize,
        *const usize,
        *const usize,
        *const usize,
        *mut *mut LmSpecs,
    ) -> LmStatus,
    pub specs_clone: unsafe extern "C" fn(*const LmSpecs, *mut *mut LmSpecs) -> LmStatus,
    pub specs_satisfies:
        unsafe extern "C" fn(*const LmSpecs, *const LmSpecs, *mut bool) -> LmStatus,
    pub specs_diff: unsafe extern "C" fn(*const LmSpecs, *const LmSpecs, *mut LmBuffer) -> LmStatus,
    pub specs_sense: unsafe extern "C" fn(*const LmSpecs, *mut LmSense) -> LmStatus,
    pub specs_vtypes: unsafe extern "C" fn(*const LmSpecs, *mut LmVtypeBuffer) -> LmStatus,
    pub specs_ctypes: unsafe extern "C" fn(*const LmSpecs, *mut LmCtypeBuffer) -> LmStatus,
    pub specs_max_degree: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
    pub specs_max_constraint_degree: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
    pub specs_max_num_variables: unsafe extern "C" fn(*const LmSpecs, *mut usize) -> LmStatus,
    // ///// //
    // Bound //
    // ///// //
    pub bound_is_bounded: unsafe extern "C" fn(*const LmBound, *mut bool) -> LmStatus,
    pub bound_value: unsafe extern "C" fn(*const LmBound, *mut f64) -> LmStatus,
    pub bound_eq: unsafe extern "C" fn(*const LmBound, *const LmBound, *mut bool) -> LmStatus,
    // ////// //
    // Bounds //
    // ////// //
    pub bounds_free: unsafe extern "C" fn(*mut LmBounds),
    pub bounds_new: unsafe extern "C" fn(LmBound, LmBound, *mut *mut LmBounds) -> LmStatus,
    pub bounds_binary: unsafe extern "C" fn(*mut *mut LmBounds) -> LmStatus,
    pub bounds_spin: unsafe extern "C" fn(*mut *mut LmBounds) -> LmStatus,
    pub bounds_integer: unsafe extern "C" fn(*mut *mut LmBounds) -> LmStatus,
    pub bounds_real: unsafe extern "C" fn(*mut *mut LmBounds) -> LmStatus,
    pub bounds_default_for: unsafe extern "C" fn(LmVtype, *mut *mut LmBounds) -> LmStatus,
    pub bounds_clone: unsafe extern "C" fn(*const LmBounds, *mut *mut LmBounds) -> LmStatus,
    pub bounds_eq: unsafe extern "C" fn(*const LmBounds, *const LmBounds, *mut bool) -> LmStatus,
    pub bounds_lower: unsafe extern "C" fn(*const LmBounds, *mut LmBound) -> LmStatus,
    pub bounds_upper: unsafe extern "C" fn(*const LmBounds, *mut LmBound) -> LmStatus,
    pub bounds_eval: unsafe extern "C" fn(*const LmBounds, f64, *const f64, *mut bool) -> LmStatus,
    // ////// //
    // LazyBounds //
    // ////// //
    pub lazy_bounds_free: unsafe extern "C" fn(*mut LmLazyBounds),
    pub lazy_bounds_default: extern "C" fn() -> *mut LmLazyBounds,
    pub lazy_bounds_new:
        unsafe extern "C" fn(*const LmBound, *const LmBound, *mut *mut LmLazyBounds) -> LmStatus,
    pub lazy_bounds_concretize:
        unsafe extern "C" fn(*const LmLazyBounds, LmVtype, *mut *mut LmBounds) -> LmStatus,
    pub lazy_bounds_clone:
        unsafe extern "C" fn(*const LmLazyBounds, *mut *mut LmLazyBounds) -> LmStatus,
    pub lazy_bounds_lower: unsafe extern "C" fn(*const LmLazyBounds, *mut LmBound) -> LmStatus,
    pub lazy_bounds_upper: unsafe extern "C" fn(*const LmLazyBounds, *mut LmBound) -> LmStatus,
    // ////// //
    // Linear //
    // ////// //
    pub linear_free: unsafe extern "C" fn(*mut LmLinear),
    pub linear_default: extern "C" fn() -> *mut LmLinear,
    pub linear_for_var: unsafe extern "C" fn(u32, f64, *mut *mut LmLinear) -> LmStatus,
    pub linear_for_vars: unsafe extern "C" fn(u32, f64, u32, f64, *mut *mut LmLinear) -> LmStatus,
    pub linear_clone: unsafe extern "C" fn(*const LmLinear, *mut *mut LmLinear) -> LmStatus,
    pub linear_eq: unsafe extern "C" fn(*const LmLinear, *const LmLinear, *mut bool) -> LmStatus,
    pub linear_len: unsafe extern "C" fn(*const LmLinear, *mut usize) -> LmStatus,
    pub linear_is_zero: unsafe extern "C" fn(*const LmLinear, *mut bool) -> LmStatus,
    pub linear_is_empty: unsafe extern "C" fn(*const LmLinear, *mut bool) -> LmStatus,
    pub linear_items: unsafe extern "C" fn(*const LmLinear, *mut LmLinearItemBuffer) -> LmStatus,
    pub linear_index: unsafe extern "C" fn(*const LmLinear, u32, *mut f64) -> LmStatus,
    pub linear_add_entry: unsafe extern "C" fn(*mut LmLinear, u32, f64) -> LmStatus,
    pub linear_add_linear: unsafe extern "C" fn(*mut LmLinear, *const LmLinear) -> LmStatus,
    pub linear_mul_bias: unsafe extern "C" fn(*mut LmLinear, f64) -> LmStatus,
    pub linear_set_bias: unsafe extern "C" fn(*mut LmLinear, u32, f64) -> LmStatus,
    pub linear_neg: unsafe extern "C" fn(*const LmLinear, *mut *mut LmLinear) -> LmStatus,
    // ///////// //
    // Quadratic //
    // ///////// //
    pub quadratic_free: unsafe extern "C" fn(*mut LmQuadratic),
    pub quadratic_default: extern "C" fn() -> *mut LmQuadratic,
    pub quadratic_clone:
        unsafe extern "C" fn(*const LmQuadratic, *mut *mut LmQuadratic) -> LmStatus,
    pub quadratic_eq:
        unsafe extern "C" fn(*const LmQuadratic, *const LmQuadratic, *mut bool) -> LmStatus,
    pub quadratic_len: unsafe extern "C" fn(*const LmQuadratic, *mut usize) -> LmStatus,
    pub quadratic_is_zero: unsafe extern "C" fn(*const LmQuadratic, *mut bool) -> LmStatus,
    pub quadratic_is_empty: unsafe extern "C" fn(*const LmQuadratic, *mut bool) -> LmStatus,
    pub quadratic_items:
        unsafe extern "C" fn(*const LmQuadratic, *mut LmQuadraticItemBuffer) -> LmStatus,
    pub quadratic_index: unsafe extern "C" fn(*const LmQuadratic, u32, u32, *mut f64) -> LmStatus,
    pub quadratic_add_entry: unsafe extern "C" fn(*mut LmQuadratic, u32, u32, f64) -> LmStatus,
    pub quadratic_add_quadratic:
        unsafe extern "C" fn(*mut LmQuadratic, *const LmQuadratic) -> LmStatus,
    pub quadratic_mul_bias: unsafe extern "C" fn(*mut LmQuadratic, f64) -> LmStatus,
    pub quadratic_clean: unsafe extern "C" fn(*mut LmQuadratic) -> LmStatus,
    pub quadratic_push_back_empty: unsafe extern "C" fn(*mut LmQuadratic, u32) -> LmStatus,
    pub quadratic_insert_empty: unsafe extern "C" fn(*mut LmQuadratic, usize, u32) -> LmStatus,
    pub quadratic_set_bias: unsafe extern "C" fn(*mut LmQuadratic, u32, u32, f64) -> LmStatus,
    pub quadratic_neg: unsafe extern "C" fn(*const LmQuadratic, *mut *mut LmQuadratic) -> LmStatus,
    // /////////// //
    // HigherOrder //
    // /////////// //
    pub higher_order_free: unsafe extern "C" fn(*mut LmHigherOrder),
    pub higher_order_default: extern "C" fn() -> *mut LmHigherOrder,
    pub higher_order_with_capacity: extern "C" fn(usize) -> *mut LmHigherOrder,
    pub higher_order_clone:
        unsafe extern "C" fn(*const LmHigherOrder, *mut *mut LmHigherOrder) -> LmStatus,
    pub higher_order_eq:
        unsafe extern "C" fn(*const LmHigherOrder, *const LmHigherOrder, *mut bool) -> LmStatus,
    pub higher_order_len: unsafe extern "C" fn(*const LmHigherOrder, *mut usize) -> LmStatus,
    pub higher_order_is_zero: unsafe extern "C" fn(*const LmHigherOrder, *mut bool) -> LmStatus,
    pub higher_order_is_empty: unsafe extern "C" fn(*const LmHigherOrder, *mut bool) -> LmStatus,
    pub higher_order_items:
        unsafe extern "C" fn(*const LmHigherOrder, *mut LmHigherOrderItemBuffer) -> LmStatus,
    pub higher_order_keys: unsafe extern "C" fn(*const LmHigherOrder, *mut LmStrBuffer) -> LmStatus,
    pub higher_order_degree: unsafe extern "C" fn(*const LmHigherOrder, *mut usize) -> LmStatus,
    pub higher_order_index:
        unsafe extern "C" fn(*const LmHigherOrder, *const u32, usize, *mut f64) -> LmStatus,
    pub higher_order_index_key:
        unsafe extern "C" fn(*const LmHigherOrder, *const c_char, *mut f64) -> LmStatus,
    pub higher_order_add_entry:
        unsafe extern "C" fn(*mut LmHigherOrder, *const u32, usize, f64) -> LmStatus,
    pub higher_order_add_higher_order:
        unsafe extern "C" fn(*mut LmHigherOrder, *const LmHigherOrder) -> LmStatus,
    pub higher_order_mul_bias: unsafe extern "C" fn(*mut LmHigherOrder, f64) -> LmStatus,
    pub higher_order_clean: unsafe extern "C" fn(*mut LmHigherOrder) -> LmStatus,
    pub higher_order_set_bias:
        unsafe extern "C" fn(*mut LmHigherOrder, *const u32, usize, f64) -> LmStatus,
    pub higher_order_set_bias_key:
        unsafe extern "C" fn(*mut LmHigherOrder, *const c_char, f64) -> LmStatus,
    pub higher_order_neg:
        unsafe extern "C" fn(*const LmHigherOrder, *mut *mut LmHigherOrder) -> LmStatus,
    // ///// //
    // Timer //
    // ///// //
    pub timer_free: unsafe extern "C" fn(*mut LmTimer),
    pub timer_start: extern "C" fn() -> *mut LmTimer,
    pub timer_stop: unsafe extern "C" fn(*const LmTimer, *mut *mut LmTiming) -> LmStatus,
    // ////// //
    // Timing //
    // ////// //
    pub timing_free: unsafe extern "C" fn(*mut LmTiming),
    pub timing_new: unsafe extern "C" fn(u64, u64, *const f64, *mut *mut LmTiming) -> LmStatus,
    pub timing_clone: unsafe extern "C" fn(*const LmTiming, *mut *mut LmTiming) -> LmStatus,
    pub timing_eq: unsafe extern "C" fn(*const LmTiming, *const LmTiming, *mut bool) -> LmStatus,
    pub timing_total_ns: unsafe extern "C" fn(*const LmTiming, *mut u64) -> LmStatus,
    pub timing_start_ns: unsafe extern "C" fn(*const LmTiming, *mut u64) -> LmStatus,
    pub timing_end_ns: unsafe extern "C" fn(*const LmTiming, *mut u64) -> LmStatus,
    pub timing_qpu: unsafe extern "C" fn(*const LmTiming, *mut f64) -> LmStatus,
}

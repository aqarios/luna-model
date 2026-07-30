use std::os::raw::c_char;

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

/// Opaque handle to a bulk per-sample solution lookup table.
///
/// Built once from an [`LmEnv`] and an [`LmSol`] with [`lm_sol_lookup_new`], then
/// reused across many samples: [`lm_sol_lookup_update`] loads one sample row's
/// values by variable id, and [`lm_sol_lookup_write_sol`] writes them back. This
/// avoids one ABI round-trip per value in hot per-sample loops (e.g. samplers).
/// Release with [`lm_sol_lookup_free`].
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmSolutionLookup {
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

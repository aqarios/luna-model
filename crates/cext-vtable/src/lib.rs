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
/// from serialized bytes with [`lm_solution_deserialize`]. Release with
/// [`lm_solution_free`].
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmSolution {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel environment.
///
/// Created by [`lm_env_default`] and released with
/// [`lm_env_free`]. The contents are intentionally hidden from C.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmEnvironment {
    _private: [u8; 0],
}

/// Opaque handle to a LunaModel variable reference.
///
/// cbindgen:opaque
#[repr(C)]
pub struct LmVref {
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
    /// See [`crate::buffer::lm_vref_buffer_free`].
    pub vref_buffer_free: unsafe extern "C" fn(LmVrefBuffer),
    // ///// //
    // MODEL //
    // ///// //
    /// See [`crate::model::lm_model_default`].
    pub model_default: extern "C" fn() -> *mut LmModel,
    /// See [`crate::model::lm_model_new`].
    pub model_new:
        unsafe extern "C" fn(*const c_char, *const LmSense, *mut *mut LmModel) -> LmStatus,
    pub model_with_env: unsafe extern "C" fn(
        *const c_char,
        *const LmSense,
        *const LmEnvironment,
        *mut *mut LmModel,
    ) -> LmStatus,
    /// See [`crate::model::lm_model_clone`].
    pub model_clone: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    /// See [`crate::model::lm_model_free`].
    pub model_free: unsafe extern "C" fn(*mut LmModel),
    /// See [`crate::model::lm_model_serialize`].
    pub model_serialize: unsafe extern "C" fn(*const LmModel, *mut LmBuffer) -> LmStatus,
    /// See [`crate::model::lm_model_deserialize`].
    pub model_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmModel) -> LmStatus,
    /// See [`crate::model::lm_model_name`].
    pub model_name: unsafe extern "C" fn(*const LmModel, *mut LmBuffer) -> LmStatus,
    /// See [`crate::model::lm_model_vtypes`].
    pub model_vtypes: unsafe extern "C" fn(*const LmModel, *mut LmVtypeBuffer) -> LmStatus,
    /// See [`crate::model::lm_model_num_variables`].
    pub model_num_variables: unsafe extern "C" fn(*const LmModel, *mut usize) -> LmStatus,
    /// See [`crate::model::lm_model_vars`].
    pub model_vars: unsafe extern "C" fn(*const LmModel, *mut LmVrefBuffer) -> LmStatus,
    /// See [`crate::model::lm_model_var`].
    pub model_var:
        unsafe extern "C" fn(*const LmModel, *const c_char, *mut *mut LmVref) -> LmStatus,
    // //////// //
    // SOLUTION //
    // //////// //
    /// See [`crate::solution::lm_solution_deserialize`].
    pub solution_deserialize:
        unsafe extern "C" fn(*const u8, usize, *mut *mut LmSolution) -> LmStatus,
    /// See [`crate::solution::lm_solution_free`].
    pub solution_free: unsafe extern "C" fn(*mut LmSolution),
}

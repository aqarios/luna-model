//! The C API function-pointer table.
//!
//! [`LUNA_MODEL_C_API`] is a single `#[repr(C)]` table of every exported entry
//! point plus the ABI version. The standalone library calls the `lm_*` symbols
//! directly; the Python wheel instead publishes a pointer to this table through a
//! `PyCapsule` (`luna_model._lm._C_API`), so a separately compiled extension can
//! call LunaModel through these pointers without linking a second copy of the core.

use std::os::raw::c_char;

use crate::CAPI_ABI_VERSION;
use crate::buffer::LmBuffer;
use crate::error::LmStatus;
use crate::model::LmModel;
use crate::solution::LmSolution;

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

    /// See [`crate::model::lm_model_new`].
    pub model_new: extern "C" fn() -> *mut LmModel,
    /// See [`crate::model::lm_model_clone`].
    pub model_clone: unsafe extern "C" fn(*const LmModel, *mut *mut LmModel) -> LmStatus,
    /// See [`crate::model::lm_model_free`].
    pub model_free: unsafe extern "C" fn(*mut LmModel),
    /// See [`crate::model::lm_model_serialize`].
    pub model_serialize: unsafe extern "C" fn(*const LmModel, *mut LmBuffer) -> LmStatus,
    /// See [`crate::model::lm_model_deserialize`].
    pub model_deserialize: unsafe extern "C" fn(*const u8, usize, *mut *mut LmModel) -> LmStatus,

    /// See [`crate::solution::lm_solution_deserialize`].
    pub solution_deserialize:
        unsafe extern "C" fn(*const u8, usize, *mut *mut LmSolution) -> LmStatus,
    /// See [`crate::solution::lm_solution_free`].
    pub solution_free: unsafe extern "C" fn(*mut LmSolution),
}

/// The singleton C API table exposed to consumers (directly or via the capsule).
pub static LUNA_MODEL_C_API: LunaModelCApi = LunaModelCApi {
    abi_version: CAPI_ABI_VERSION,
    version: crate::version::lm_version,
    last_error_message: crate::error::lm_last_error_message,
    buffer_free: crate::buffer::lm_buffer_free,
    model_new: crate::model::lm_model_new,
    model_clone: crate::model::lm_model_clone,
    model_free: crate::model::lm_model_free,
    model_serialize: crate::model::lm_model_serialize,
    model_deserialize: crate::model::lm_model_deserialize,
    solution_deserialize: crate::solution::lm_solution_deserialize,
    solution_free: crate::solution::lm_solution_free,
};

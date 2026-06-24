//! The C API function-pointer table.
//!
//! [`LUNA_MODEL_C_API`] fills the `LunaModelCApi` table (defined in
//! `lunamodel-cext-vtable`) with this crate's `extern "C"` entry points. The
//! standalone library calls the `lm_*` symbols directly; the Python wheel publishes
//! a pointer to this table through a `PyCapsule` (`luna_model._lm._C_API`).

use crate::{CAPI_ABI_VERSION, LunaModelCApi};

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

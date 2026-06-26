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
    // Buffer
    buffer_free: crate::buffer::lm_buffer_free,
    vtype_buffer_free: crate::buffer::lm_vtype_buffer_free,
    vref_buffer_free: crate::buffer::lm_vref_buffer_free,
    // Model
    model_free: crate::model::lm_model_free,
    // Model -- creation
    model_default: crate::model::lm_model_default,
    model_new: crate::model::lm_model_new,
    model_with_env: crate::model::lm_model_with_env,
    // Model -- general
    model_clone: crate::model::lm_model_clone,
    // Model -- access
    model_name: crate::model::lm_model_name,
    model_vtypes: crate::model::lm_model_vtypes,
    model_num_variables: crate::model::lm_model_num_variables,
    model_vars: crate::model::lm_model_vars,
    model_var: crate::model::lm_model_var,
    // Model -- serializer
    model_serialize: crate::model::lm_model_serialize,
    model_deserialize: crate::model::lm_model_deserialize,
    // Env
    env_free: crate::env::lm_env_free,
    env_default: crate::env::lm_env_default,
    // Sol
    sol_free: crate::solution::lm_sol_free,
    // Sol -- serializer
    sol_deserialize: crate::solution::lm_sol_deserialize,
};

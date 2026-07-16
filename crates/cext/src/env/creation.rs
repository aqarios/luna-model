use std::collections::HashMap;

use indexmap::IndexMap;
use luna_model::core::{ArcEnv, Environment, prelude::Variable};
use lunamodel_cext_handle::{drop_handle, handle_from_storage};

use crate::{
    LmEnv, LmStatus, LmVar,
    error::guard,
    utils::{ref_from_c, slice_from_c, storage_out_from_c, str_from_c},
};

/// Frees an environment handle. Passing null is a no-op.
///
/// # Safety
/// `env` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_free(env: *mut LmEnv) {
    unsafe { drop_handle(env) }
}

/// Creates a new, empty environment.
///
/// Returns a non-null handle that must be released with [`lm_env_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_env_default() -> *mut LmEnv {
    handle_from_storage::<LmEnv>(ArcEnv::default())
}

/// Creates an environment from owned variable records and explicit indices.
///
/// # Safety
/// All pointer arrays must contain the stated number of valid elements and
/// `out` must be writable environment-handle storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_env_new(
    variables: *const *const LmVar,
    indices: *const u32,
    len: usize,
    lookup_names: *const *const std::ffi::c_char,
    lookup_indices: *const u32,
    lookup_len: usize,
    next_idx: u32,
    out: *mut *mut LmEnv,
) -> LmStatus {
    guard(|| {
        let variables = unsafe { slice_from_c(variables, len, "variables") }?;
        let indices = unsafe { slice_from_c(indices, len, "indices") }?;
        let mut storage = IndexMap::with_capacity(len);
        let lookup_names = unsafe { slice_from_c(lookup_names, lookup_len, "lookup_names") }?;
        let lookup_indices = unsafe { slice_from_c(lookup_indices, lookup_len, "lookup_indices") }?;
        let mut lookup = HashMap::with_capacity(lookup_len);
        for (&variable, &index) in variables.iter().zip(indices) {
            let variable = unsafe { ref_from_c(variable as *const Variable, "variable") }?.clone();
            storage.insert(index, variable);
        }
        for (&name, &index) in lookup_names.iter().zip(lookup_indices) {
            lookup.insert(
                unsafe { str_from_c(name, "lookup_name") }?.to_owned(),
                index,
            );
        }
        unsafe {
            storage_out_from_c(
                out,
                ArcEnv::from(Environment::new(storage, lookup, next_idx)),
            )
        }
    })
}

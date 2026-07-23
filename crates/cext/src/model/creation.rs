use std::ffi::c_char;

use lunamodel_core::Model;
use lunamodel_cext_handle::{drop_handle, new_handle};
use lunamodel_cext_vtable::LmEnv;

use crate::{
    LmModel, LmSense, LmStatus,
    error::guard,
    utils::sense_from_c,
    utils::{new_handle_out_from_c, opt_from_c, opt_str_from_c, storage_from_c},
};

/// Frees a model handle. Passing null is a no-op.
///
/// # Safety
/// `model` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_free(model: *mut LmModel) {
    unsafe { drop_handle(model) }
}

/// Creates a new, empty (unnamed, minimization) model.
///
/// Returns a non-null handle that must be released with [`lm_model_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_model_default() -> *mut LmModel {
    new_handle(Model::default())
}

/// Creates a new model.
///
/// `name == NULL` means use the default model name.
/// `sense == NULL` means use the default optimization sense.
/// `out` receives an owned model handle that must be freed with `lm_model_free`.
///
/// # Safety
/// `name` must be null or point to a valid NUL-terminated C string.
/// `sense` must be null or point to a readable `LmSense`.
/// `out` must point to writable `LmModel *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_new(
    name: *const c_char,
    sense: *const LmSense,
    out: *mut *mut LmModel,
) -> LmStatus {
    guard(|| {
        let name = unsafe { opt_str_from_c(name, "name") }?.map(str::to_owned);
        let sense = unsafe { opt_from_c(sense) }.map(sense_from_c);

        let model = Model::new(name, sense);
        unsafe { new_handle_out_from_c(out, model) }
    })
}

/// Creates a new model for an environment.
///
/// `name == NULL` means use the default model name.
/// `sense == NULL` means use the default optimization sense.
/// `out` receives an owned model handle that must be freed with `lm_model_free`.
///
/// # Safety
/// `name` must be null or point to a valid NUL-terminated C string.
/// `sense` must be null or point to a readable `LmSense`.
/// `env` must be a valid environment handle created by this library.
/// `out` must point to writable `LmModel *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_with_env(
    name: *const c_char,
    sense: *const LmSense,
    env: *const LmEnv,
    out: *mut *mut LmModel,
) -> LmStatus {
    guard(|| {
        let env = unsafe { storage_from_c(env, "env") }?;

        let name = unsafe { opt_str_from_c(name, "name") }?.map(str::to_owned);
        let sense = unsafe { opt_from_c(sense) }.map(sense_from_c);

        let model = Model::with_env(name, sense, env.clone());
        unsafe { new_handle_out_from_c(out, model) }
    })
}

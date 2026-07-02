use crate::utils::{drop_from_c, handle_to_c, opt_from_c, opt_str_from_c, out_from_c};
use std::ffi::c_char;

use luna_model::core::Model;
use lunamodel_cext_vtable::LmEnv;

use crate::{
    LmModel, LmSense, LmStatus, env::utils::environment_from_c, error::guard, utils::sense_from_c,
};

/// Frees a model handle. Passing null is a no-op.
///
/// # Safety
/// `model` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_free(model: *mut LmModel) {
    unsafe { drop_from_c(model as *mut Model) }
}

/// Creates a new, empty (unnamed, minimization) model.
///
/// Returns a non-null handle that must be released with [`lm_model_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_model_default() -> *mut LmModel {
    handle_to_c(Model::default())
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
        let out = unsafe { out_from_c(out) }?;

        let name = unsafe { opt_str_from_c(name, "name") }?.map(str::to_owned);
        let sense = unsafe { opt_from_c(sense) }.map(sense_from_c);

        let model = Model::new(name, sense);
        *out = handle_to_c(model);

        Ok(())
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
        let out = unsafe { out_from_c(out) }?;

        let env = unsafe { environment_from_c(env) }?;

        let name = unsafe { opt_str_from_c(name, "name") }?.map(str::to_owned);
        let sense = unsafe { opt_from_c(sense) }.map(sense_from_c);

        let model = Model::with_env(name, sense, env.clone());
        *out = handle_to_c(model);

        Ok(())
    })
}

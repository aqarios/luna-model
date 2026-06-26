use std::ffi::{CStr, c_char};

use luna_model::core::Model;
use lunamodel_cext_vtable::LmEnv;

use crate::{
    LmModel, LmSense, LmStatus,
    env::utils::environment_from_c,
    error::{CapiError, guard},
    utils::sense_from_c,
};

/// Frees a model handle. Passing null is a no-op.
///
/// # Safety
/// `model` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_free(model: *mut LmModel) {
    if !model.is_null() {
        drop(unsafe { Box::from_raw(model as *mut Model) });
    }
}

/// Creates a new, empty (unnamed, minimization) model.
///
/// Returns a non-null handle that must be released with [`lm_model_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_model_default() -> *mut LmModel {
    Box::into_raw(Box::new(Model::default())) as *mut LmModel
}

/// Creates a new model.
///
/// `name == NULL` means use the default model name.
/// `sense == NULL` means use the default optimization sense.
/// `out` receives an owned model handle that must be freed with `lm_model_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_new(
    name: *const c_char,
    sense: *const LmSense,
    out: *mut *mut LmModel,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let name = if name.is_null() {
            None
        } else {
            let cstr = unsafe { CStr::from_ptr(name) };
            Some(
                cstr.to_str()
                    .map_err(|e| {
                        CapiError::new(
                            LmStatus::InvalidArgument,
                            format!("`name` must be valid UTF-8: {e}"),
                        )
                    })?
                    .to_owned(),
            )
        };

        let sense = if sense.is_null() {
            None
        } else {
            Some(sense_from_c(unsafe { *sense }))
        };

        let model = Model::new(name, sense);
        *out = Box::into_raw(Box::new(model)) as *mut LmModel;

        Ok(())
    })
}

/// Creates a new model for an environment
///
/// `name == NULL` means use the default model name.
/// `out` receives an owned model handle that must be freed with `lm_model_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_with_env(
    name: *const c_char,
    sense: *const LmSense,
    env: *const LmEnv,
    out: *mut *mut LmModel,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let env = unsafe { environment_from_c(env) }?;

        let name = if name.is_null() {
            None
        } else {
            let cstr = unsafe { CStr::from_ptr(name) };
            Some(
                cstr.to_str()
                    .map_err(|e| {
                        CapiError::new(
                            LmStatus::InvalidArgument,
                            format!("`name` must be valid UTF-8: {e}"),
                        )
                    })?
                    .to_owned(),
            )
        };

        let sense = if sense.is_null() {
            None
        } else {
            Some(sense_from_c(unsafe { *sense }))
        };

        let model = Model::with_env(name, sense, env.clone());
        *out = Box::into_raw(Box::new(model)) as *mut LmModel;

        Ok(())
    })
}

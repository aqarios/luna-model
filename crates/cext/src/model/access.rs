use std::ffi::c_char;

use lunamodel_cext_handle::{ConstraintCollectionHandle, ExpressionHandle};

use crate::{
    LmBuffer, LmCC, LmEnv, LmExpr, LmModel, LmSense, LmSpecs, LmStatus, LmVref, LmVrefBuffer,
    LmVtype, LmVtypeBuffer,
    buffer::{buffer_from_vec, vref_buffer_from_vec, vtype_buffer_from_vec},
    error::{CapiError, guard},
    utils::{handle_to_c, out_from_c, shared_from_c, storage_out_from_c, str_from_c},
    utils::{sense_to_c, vtype_to_c},
};

/// Returns a copy of the model name as an owned UTF-8 byte buffer.
///
/// The returned buffer is written to `out` and must be released with
/// `lm_buffer_free`.
///
/// The buffer is not guaranteed to be NUL-terminated. C callers should use
/// `buffer.data` together with `buffer.len`.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_name(model: *const LmModel, out: *mut LmBuffer) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = buffer_from_vec(model.name.as_bytes().to_vec());
        Ok(())
    })
}

/// Returns the optimization sense of the model.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmSense` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_sense(model: *const LmModel, out: *mut LmSense) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = sense_to_c(model.sense);
        Ok(())
    })
}

/// Returns the environment of the model.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmEnv *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_env(model: *const LmModel, out: *mut *mut LmEnv) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        unsafe { storage_out_from_c(out, model.environment.clone()) }
    })
}

/// Returns the specs of the model.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmSpecs *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_specs(model: *const LmModel, out: *mut *mut LmSpecs) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(model.specs());
        Ok(())
    })
}

/// Returns the distinct variable types used by the model.
///
/// This is derived from both the objective and all constraints, matching
/// `Model::vtypes()` in the Rust API. The returned array is owned by the caller
/// and must be released with `lm_vtype_buffer_free`.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmVtypeBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_vtypes(
    model: *const LmModel,
    out: *mut LmVtypeBuffer,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        let vtypes = model.vtypes().map(vtype_to_c).collect::<Vec<LmVtype>>();
        *out = vtype_buffer_from_vec(vtypes);
        Ok(())
    })
}

/// Returns the number of variables in the model.
///
/// This is derived from both the objective and all constraints, matching
/// `Model::num_variables()` in the Rust API.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `uintptr_t` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_num_variables(
    model: *const LmModel,
    out: *mut usize,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = model.num_variables();
        Ok(())
    })
}

/// Returns the distinct variables used by the model.
///
/// This is derived from both the objective and all constraints, matching
/// `Model::vars()` in the Rust API. The returned array is owned by the caller
/// and must be released with `lm_vref_buffer_free`.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmVrefBuffer` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_vars(model: *const LmModel, out: *mut LmVrefBuffer) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        let vars = model.vars().map(handle_to_c).collect::<Vec<*mut LmVref>>();
        *out = vref_buffer_from_vec(vars);
        Ok(())
    })
}

/// Looks up a variable by name in the model environment.
///
/// `name` must be non-null and valid UTF-8.
/// `out` receives an owned variable handle that must be freed with
/// `lm_variable_free`.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `out` must point to writable `LmVref *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_var(
    model: *const LmModel,
    name: *const c_char,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        let name = unsafe { str_from_c(name, "name") }?;
        let var = model.var(name).map_err(CapiError::invalid_argument)?;
        *out = handle_to_c(var);
        Ok(())
    })
}

/// Returns the objective of the model.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmExpr *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_objective(
    model: *const LmModel,
    out: *mut *mut LmExpr,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?;
        unsafe { storage_out_from_c(out, ExpressionHandle::for_model(model.clone())) }
    })
}

/// Returns the constraints of the model.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmCC *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_constraints(
    model: *const LmModel,
    out: *mut *mut LmCC,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?;
        unsafe { storage_out_from_c(out, ConstraintCollectionHandle::for_model(model.clone())) }
    })
}

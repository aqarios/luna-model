use crate::utils::{
    handle_to_c, new_handle_out_from_c, out_from_c, ref_from_c, retained_handle_out_from_c,
    shared_from_c, str_from_c,
};
use std::ffi::c_char;

use luna_model::core::Bounds;
use luna_model::types::Specs;
use lunamodel_cext_vtable::LmLazyBounds;

use crate::error::{CapiError, guard};
use crate::utils::vtype_from_c;
use crate::{LmModel, LmSpecs, LmStatus, LmVref, LmVtype};

/// Clones a model into a new, independent handle written to `out`.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmModel *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_clone(model: *const LmModel, out: *mut *mut LmModel) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        unsafe { new_handle_out_from_c(out, model.clone()) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_retain(
    model: *const LmModel,
    out: *mut *mut LmModel,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?;
        unsafe { retained_handle_out_from_c(out, model.clone()) }
    })
}

/// Deep-clones a model into a new, independent handle written to `out`.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `out` must point to writable `LmModel *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_deep_clone(
    model: *const LmModel,
    out: *mut *mut LmModel,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        unsafe { new_handle_out_from_c(out, model.deep_clone()) }
    })
}

/// Returns whether the model satisfies the given specs.
///
/// This matches `Model::satisfies()` in the Rust API.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `specs` must be a valid specs handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_satisfies(
    model: *const LmModel,
    specs: *const LmSpecs,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let specs = unsafe { ref_from_c(specs as *const Specs, "specs") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = model.satisfies(specs);
        Ok(())
    })
}

/// Adds a new variable to the model.
///
/// This matches `Model::add_var()` in the Rust API.
///
/// `bounds == NULL` means use the default bounds for `vtype`.
/// `out` receives an owned variable-reference handle that must be freed with
/// `lm_vref_free`.
///
/// # Safety
/// `model` must be a valid mutable model handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `bounds`, when non-null, must be a valid bounds handle created by this library.
/// `out` must point to writable `LmVref *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_add_var(
    model: *mut LmModel,
    name: *const c_char,
    vtype: LmVtype,
    bounds: *const LmLazyBounds,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let mut model = unsafe { shared_from_c(model, "model") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        let vtype = vtype_from_c(vtype);
        let bounds = if bounds.is_null() {
            None
        } else {
            let bounds = unsafe { ref_from_c(bounds as *const Bounds, "bounds") }?;
            Some((*bounds).into())
        };
        let out = unsafe { out_from_c(out) }?;
        let var = model
            .add_var(name, vtype, bounds)
            .map_err(CapiError::invalid_argument)?;
        *out = handle_to_c(var);
        Ok(())
    })
}

/// Adds a new variable to the model with a fallback.
///
/// This matches `Model::add_var_with_fallback()` in the Rust API.
///
/// `bounds == NULL` means use the default bounds for `vtype`.
/// `enc == NULL` means use the default for generating the fallback.
/// `out` receives an owned variable-reference handle that must be freed with
/// `lm_vref_free`.
///
/// # Safety
/// `model` must be a valid mutable model handle created by this library.
/// `name` must point to a valid NUL-terminated C string.
/// `bounds`, when non-null, must be a valid bounds handle created by this library.
/// `out` must point to writable `LmVref *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_add_var_with_fallback(
    model: *mut LmModel,
    name: *const c_char,
    vtype: LmVtype,
    bounds: *const LmLazyBounds,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let mut model = unsafe { shared_from_c(model, "model") }?.write_arc();
        let name = unsafe { str_from_c(name, "name") }?;
        let vtype = vtype_from_c(vtype);
        let bounds = if bounds.is_null() {
            None
        } else {
            let bounds = unsafe { ref_from_c(bounds as *const Bounds, "bounds") }?;
            Some((*bounds).into())
        };
        let out = unsafe { out_from_c(out) }?;
        let var = model
            .add_var_with_fallback(name, vtype, bounds, None)
            .map_err(CapiError::invalid_argument)?;
        *out = handle_to_c(var);
        Ok(())
    })
}

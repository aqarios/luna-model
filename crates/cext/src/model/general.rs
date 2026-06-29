use std::ffi::{CStr, c_char};

use luna_model::core::{Bounds, Model};
use luna_model::types::Specs;

use crate::error::{CapiError, guard};
use crate::utils::vtype_from_c;
use crate::{LmBounds, LmModel, LmSpecs, LmStatus, LmVref, LmVtype};

/// Clones a model into a new, independent handle written to `out`.
///
/// # Safety
/// `model` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_clone(model: *const LmModel, out: *mut *mut LmModel) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(model.clone())) as *mut LmModel;
        Ok(())
    })
}

/// Deep-clones a model into a new, independent handle written to `out`.
///
/// # Safety
/// `model` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_deep_clone(
    model: *const LmModel,
    out: *mut *mut LmModel,
) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(model.deep_clone())) as *mut LmModel;
        Ok(())
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
        let model = unsafe { (model as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;

        let specs = unsafe { (specs as *const Specs).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("specs"))?;

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

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
/// `vtype` must point to a valid `LmVtype`.
/// `bounds`, when non-null, must be a valid bounds handle created by this library.
/// `out` must point to writable `LmVref *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_add_var(
    model: *mut LmModel,
    name: *const c_char,
    vtype: *const LmVtype,
    bounds: *const LmBounds,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *mut Model).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;

        if name.is_null() {
            return Err(CapiError::null_pointer("name"));
        }

        let name = unsafe { CStr::from_ptr(name) }.to_str().map_err(|e| {
            CapiError::new(
                LmStatus::InvalidArgument,
                format!("`name` must be valid UTF-8: {e}"),
            )
        })?;

        let vtype = unsafe { vtype.as_ref() }.ok_or_else(|| CapiError::null_pointer("vtype"))?;
        let vtype = vtype_from_c(*vtype);

        let bounds = if bounds.is_null() {
            None
        } else {
            let bounds = unsafe { (bounds as *const Bounds).as_ref() }
                .ok_or_else(|| CapiError::null_pointer("bounds"))?;
            Some((*bounds).into())
        };

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let var = model
            .add_var(name, vtype, bounds)
            .map_err(|e| CapiError::new(LmStatus::InvalidArgument, e.to_string()))?;

        *out = Box::into_raw(Box::new(var)) as *mut LmVref;

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
/// `vtype` must point to a valid `LmVtype`.
/// `bounds`, when non-null, must be a valid bounds handle created by this library.
/// `out` must point to writable `LmVref *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_add_var_with_fallback(
    model: *mut LmModel,
    name: *const c_char,
    vtype: *const LmVtype,
    bounds: *const LmBounds,
    out: *mut *mut LmVref,
) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *mut Model).as_mut() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;

        if name.is_null() {
            return Err(CapiError::null_pointer("name"));
        }

        let name = unsafe { CStr::from_ptr(name) }.to_str().map_err(|e| {
            CapiError::new(
                LmStatus::InvalidArgument,
                format!("`name` must be valid UTF-8: {e}"),
            )
        })?;

        let vtype = unsafe { vtype.as_ref() }.ok_or_else(|| CapiError::null_pointer("vtype"))?;
        let vtype = vtype_from_c(*vtype);

        let bounds = if bounds.is_null() {
            None
        } else {
            let bounds = unsafe { (bounds as *const Bounds).as_ref() }
                .ok_or_else(|| CapiError::null_pointer("bounds"))?;
            Some((*bounds).into())
        };

        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let var = model
            .add_var_with_fallback(name, vtype, bounds, None)
            .map_err(|e| CapiError::new(LmStatus::InvalidArgument, e.to_string()))?;

        *out = Box::into_raw(Box::new(var)) as *mut LmVref;

        Ok(())
    })
}

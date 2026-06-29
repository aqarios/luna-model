use luna_model::core::{Model, prelude::ContentEquality};

use crate::{
    LmModel, LmStatus,
    error::{CapiError, guard},
};

/// Compares two models for equality.
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `other` must be a valid model handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_eq(
    model: *const LmModel,
    other: *const LmModel,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let other = unsafe { (other as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = model.eq(other);
        Ok(())
    })
}

/// Compares two models for equal contents (ignoring different environments).
///
/// # Safety
/// `model` must be a valid model handle created by this library.
/// `other` must be a valid model handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_model_equal_contents(
    model: *const LmModel,
    other: *const LmModel,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let model = unsafe { (model as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("model"))?;
        let other = unsafe { (other as *const Model).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = model.equal_contents(other);
        Ok(())
    })
}

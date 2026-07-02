use crate::utils::{out_from_c, ref_from_c};
use luna_model::core::{Model, prelude::ContentEquality};

use crate::{LmModel, LmStatus, error::guard};

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
        let model = unsafe { ref_from_c(model as *const Model, "model") }?;
        let other = unsafe { ref_from_c(other as *const Model, "other") }?;
        let out = unsafe { out_from_c(out) }?;
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
        let model = unsafe { ref_from_c(model as *const Model, "model") }?;
        let other = unsafe { ref_from_c(other as *const Model, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = model.equal_contents(other);
        Ok(())
    })
}

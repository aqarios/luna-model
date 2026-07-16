use luna_model::core::prelude::ContentEquality;

use crate::{
    LmModel, LmStatus,
    error::guard,
    utils::{out_from_c, shared_from_c},
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
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let other = unsafe { shared_from_c(other, "other") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = model.eq(&other);
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
        let model = unsafe { shared_from_c(model, "model") }?.read_arc();
        let other = unsafe { shared_from_c(other, "other") }?.read_arc();
        let out = unsafe { out_from_c(out) }?;
        *out = model.equal_contents(&other);
        Ok(())
    })
}

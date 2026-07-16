use luna_model::core::prelude::Quadratic;

use crate::{
    LmQuadratic, LmStatus,
    error::guard,
    utils::{handle_to_c, out_from_c, ref_from_c},
};

/// Compares two quadratic terms for equality.
///
/// # Safety
/// `quadratic` must be a valid quadratic handle created by this library.
/// `other` must be a valid quadratic handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_eq(
    quadratic: *const LmQuadratic,
    other: *const LmQuadratic,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { ref_from_c(quadratic as *const Quadratic, "quadratic") }?;
        let other = unsafe { ref_from_c(other as *const Quadratic, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = quadratic.eq(other);
        Ok(())
    })
}

/// Clones a quadratic term into a new, independent handle written to `out`.
///
/// # Safety
/// `quadratic` must be a valid quadratic handle created by this library.
/// `out` must point to writable `LmQuadratic *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_clone(
    quadratic: *const LmQuadratic,
    out: *mut *mut LmQuadratic,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { ref_from_c(quadratic as *const Quadratic, "quadratic") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(quadratic.clone());
        Ok(())
    })
}

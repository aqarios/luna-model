use luna_model::core::prelude::Quadratic;

use crate::{
    LmQuadratic, LmStatus,
    error::{CapiError, guard},
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
        let quadratic = unsafe { (quadratic as *const Quadratic).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        let other = unsafe { (other as *const Quadratic).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = quadratic.eq(other);
        Ok(())
    })
}

/// Clones a quadratic term into a new, independent handle written to `out`.
///
/// # Safety
/// `quadratic` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_quadratic_clone(
    quadratic: *const LmQuadratic,
    out: *mut *mut LmQuadratic,
) -> LmStatus {
    guard(|| {
        let quadratic = unsafe { (quadratic as *const Quadratic).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("quadratic"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(quadratic.clone())) as *mut LmQuadratic;
        Ok(())
    })
}

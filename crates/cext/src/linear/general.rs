use luna_model::core::prelude::Linear;

use crate::{
    LmLinear, LmStatus,
    error::{CapiError, guard},
};

/// Compares two linear terms for equality.
///
/// # Safety
/// `linear` must be a valid linear handle created by this library.
/// `other` must be a valid linear handle created by this library.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_eq(
    linear: *const LmLinear,
    other: *const LmLinear,
    out: *mut bool,
) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *const Linear).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        let other = unsafe { (other as *const Linear).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("other"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = linear.eq(other);
        Ok(())
    })
}

/// Clones a linear term into a new, independent handle written to `out`.
///
/// # Safety
/// `linear` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_linear_clone(
    linear: *const LmLinear,
    out: *mut *mut LmLinear,
) -> LmStatus {
    guard(|| {
        let linear = unsafe { (linear as *const Linear).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("linear"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(linear.clone())) as *mut LmLinear;
        Ok(())
    })
}

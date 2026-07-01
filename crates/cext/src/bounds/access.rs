use luna_model::core::prelude::Bounds;

use crate::{
    LmBound, LmBounds, LmStatus,
    error::{CapiError, guard},
    utils::bound_to_c,
};

/// Returns a copy of the bounds' lower bound.
///
/// # Safety
/// `bounds` must be a valid bounds handle created by this library.
/// `out` must point to writable `LmBound` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_lower(
    bounds: *const LmBounds,
    out: *mut *mut LmBound,
) -> LmStatus {
    guard(|| {
        let bounds = unsafe { (bounds as *const Bounds).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("bounds"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(bound_to_c(bounds.lower))) as *mut LmBound;
        Ok(())
    })
}

/// Returns a copy of the bounds' upper bound.
///
/// # Safety
/// `bounds` must be a valid bounds handle created by this library.
/// `out` must point to writable `LmBound` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_upper(
    bounds: *const LmBounds,
    out: *mut *mut LmBound,
) -> LmStatus {
    guard(|| {
        let bounds = unsafe { (bounds as *const Bounds).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("bounds"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(bound_to_c(bounds.upper))) as *mut LmBound;
        Ok(())
    })
}

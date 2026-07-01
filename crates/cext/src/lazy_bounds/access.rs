use luna_model::core::prelude::LazyBounds;

use crate::{
    LmBound, LmLazyBounds, LmStatus,
    error::{CapiError, guard},
    utils::bound_to_c,
};

/// Returns a copy of the lazy-bounds' lower bound.
///
/// # Safety
/// `lazy_bounds` must be a valid lazy-bounds handle created by this library.
/// `out` must point to writable `LmBound` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_lower(
    lazy_bounds: *const LmLazyBounds,
    out: *mut *mut LmBound,
) -> LmStatus {
    guard(|| {
        let bounds = unsafe { (lazy_bounds as *const LazyBounds).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("lazy_bounds"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = bounds
            .lower
            .map(|b| Box::into_raw(Box::new(bound_to_c(b))) as *mut LmBound)
            .unwrap_or(std::ptr::null_mut());
        Ok(())
    })
}

/// Returns a copy of the lazy-bounds' upper bound.
///
/// # Safety
/// `lazy_bounds` must be a valid lazy-bounds handle created by this library.
/// `out` must point to writable `LmBound` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_upper(
    lazy_bounds: *const LmLazyBounds,
    out: *mut *mut LmBound,
) -> LmStatus {
    guard(|| {
        let bounds = unsafe { (lazy_bounds as *const LazyBounds).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("lazy_bounds"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = bounds
            .upper
            .map(|b| Box::into_raw(Box::new(bound_to_c(b))) as *mut LmBound)
            .unwrap_or(std::ptr::null_mut());
        Ok(())
    })
}

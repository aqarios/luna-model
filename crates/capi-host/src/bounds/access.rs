use crate::utils::{out_from_c, ref_from_c};
use lunamodel_core::Bounds;

use crate::{LmBound, LmBounds, LmStatus, error::guard, utils::bound_to_c};

/// Returns a copy of the bounds' lower bound.
///
/// # Safety
/// `bounds` must be a valid bounds handle created by this library.
/// `out` must point to writable `LmBound` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_lower(bounds: *const LmBounds, out: *mut LmBound) -> LmStatus {
    guard(|| {
        let bounds = unsafe { ref_from_c(bounds as *const Bounds, "bounds") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = bound_to_c(bounds.lower);
        Ok(())
    })
}

/// Returns a copy of the bounds' upper bound.
///
/// # Safety
/// `bounds` must be a valid bounds handle created by this library.
/// `out` must point to writable `LmBound` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_upper(bounds: *const LmBounds, out: *mut LmBound) -> LmStatus {
    guard(|| {
        let bounds = unsafe { ref_from_c(bounds as *const Bounds, "bounds") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = bound_to_c(bounds.upper);
        Ok(())
    })
}

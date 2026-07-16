use crate::utils::{handle_to_c, out_from_c, ref_from_c};
use luna_model::core::prelude::Bounds;

use crate::{LmBounds, LmStatus, error::guard};

/// Clones a bounds into a new, independent handle written to `out`.
///
/// # Safety
/// `bounds` must be a valid bounds handle created by this library.
/// `out` must point to writable `LmBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_clone(
    bounds: *const LmBounds,
    out: *mut *mut LmBounds,
) -> LmStatus {
    guard(|| {
        let bounds = unsafe { ref_from_c(bounds as *const Bounds, "bounds") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(*bounds);
        Ok(())
    })
}

/// Compares two bounds values.
///
/// # Safety
/// `bounds` and `other` must be valid handles and `out` must be writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_eq(
    bounds: *const LmBounds,
    other: *const LmBounds,
    out: *mut bool,
) -> LmStatus {
    crate::error::guard(|| {
        let bounds = unsafe { ref_from_c(bounds as *const Bounds, "bounds") }?;
        let other = unsafe { ref_from_c(other as *const Bounds, "other") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = bounds == other;
        Ok(())
    })
}

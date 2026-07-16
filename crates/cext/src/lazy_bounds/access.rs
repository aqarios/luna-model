use luna_model::core::prelude::LazyBounds;

use crate::{
    LmBound, LmLazyBounds, LmStatus,
    error::{CapiError, guard},
    utils::bound_to_c,
    utils::{out_from_c, ref_from_c},
};

/// Returns a copy of the lazy-bounds' lower bound.
///
/// Returns [`LmStatus::Absent`] (leaving `out` untouched) when no explicit
/// lower bound is set.
///
/// # Safety
/// `lazy_bounds` must be a valid lazy-bounds handle created by this library.
/// `out` must point to writable `LmBound` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_lower(
    lazy_bounds: *const LmLazyBounds,
    out: *mut LmBound,
) -> LmStatus {
    guard(|| {
        let bounds = unsafe { ref_from_c(lazy_bounds as *const LazyBounds, "lazy_bounds") }?;
        let out = unsafe { out_from_c(out) }?;
        match bounds.lower {
            Some(b) => {
                *out = bound_to_c(b);
                Ok(())
            }
            None => Err(CapiError::new(
                LmStatus::Absent,
                "lazy-bounds has no explicit lower bound",
            )),
        }
    })
}

/// Returns a copy of the lazy-bounds' upper bound.
///
/// Returns [`LmStatus::Absent`] (leaving `out` untouched) when no explicit
/// upper bound is set.
///
/// # Safety
/// `lazy_bounds` must be a valid lazy-bounds handle created by this library.
/// `out` must point to writable `LmBound` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_upper(
    lazy_bounds: *const LmLazyBounds,
    out: *mut LmBound,
) -> LmStatus {
    guard(|| {
        let bounds = unsafe { ref_from_c(lazy_bounds as *const LazyBounds, "lazy_bounds") }?;
        let out = unsafe { out_from_c(out) }?;
        match bounds.upper {
            Some(b) => {
                *out = bound_to_c(b);
                Ok(())
            }
            None => Err(CapiError::new(
                LmStatus::Absent,
                "lazy-bounds has no explicit upper bound",
            )),
        }
    })
}

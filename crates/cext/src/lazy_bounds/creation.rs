use luna_model::core::prelude::LazyBounds;

use crate::{
    LmBound, LmLazyBounds, LmStatus,
    error::{CapiError, guard},
    utils::bound_from_c,
};

/// Frees a lazy-bounds handle. Passing null is a no-op.
///
/// # Safety
/// `lazy_bounds` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_free(lazy_bounds: *mut LmLazyBounds) {
    if !lazy_bounds.is_null() {
        drop(unsafe { Box::from_raw(lazy_bounds as *mut LazyBounds) });
    }
}

/// Creates a default lazy-bounds.
#[unsafe(no_mangle)]
pub extern "C" fn lm_lazy_bounds_default() -> *mut LmLazyBounds {
    Box::into_raw(Box::new(LazyBounds::default())) as *mut LmLazyBounds
}

/// Creates a new lazy-bounds.
///
/// # Safety
/// `lower == NULL`: use the default lower bounds when this lazy-bounds is concretized for a LmVtype.
/// `upper == NULL`: use the default lower bounds when this lazy-bounds is concretized for a LmVtype.
/// `out`  must point to writable `LmBounds` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_new(
    lower: *const LmBound,
    upper: *const LmBound,
    out: *mut *mut LmLazyBounds,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;

        let lower = unsafe { lower.as_ref() }
            .copied()
            .and_then(|b| Some(bound_from_c(b)));
        let upper = unsafe { upper.as_ref() }
            .copied()
            .and_then(|b| Some(bound_from_c(b)));

        let bounds = LazyBounds::new(lower, upper);
        *out = Box::into_raw(Box::new(bounds)) as *mut LmLazyBounds;
        Ok(())
    })
}

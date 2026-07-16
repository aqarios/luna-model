use luna_model::core::prelude::LazyBounds;

use crate::{
    LmBound, LmLazyBounds, LmStatus,
    error::guard,
    utils::bound_from_c,
    utils::{drop_from_c, handle_to_c, opt_from_c, out_from_c},
};

/// Frees a lazy-bounds handle. Passing null is a no-op.
///
/// # Safety
/// `lazy_bounds` must originate from this library and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_free(lazy_bounds: *mut LmLazyBounds) {
    unsafe { drop_from_c(lazy_bounds as *mut LazyBounds) }
}

/// Creates a lazy-bounds with both endpoints unset.
///
/// Returns a non-null handle that must be released with [`lm_lazy_bounds_free`].
#[unsafe(no_mangle)]
pub extern "C" fn lm_lazy_bounds_default() -> *mut LmLazyBounds {
    handle_to_c(LazyBounds::default())
}

/// Creates a new lazy-bounds.
///
/// `lower == NULL` / `upper == NULL` mean "unset": the type-specific default
/// is used for that endpoint when the lazy-bounds is concretized for a vtype.
///
/// On success, `out` receives an owned handle that must be released with
/// [`lm_lazy_bounds_free`].
///
/// # Safety
/// `lower` must be null or point to a readable `LmBound`.
/// `upper` must be null or point to a readable `LmBound`.
/// `out` must point to writable `LmLazyBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_new(
    lower: *const LmBound,
    upper: *const LmBound,
    out: *mut *mut LmLazyBounds,
) -> LmStatus {
    guard(|| {
        let out = unsafe { out_from_c(out) }?;

        let lower = unsafe { opt_from_c(lower) }.map(bound_from_c);
        let upper = unsafe { opt_from_c(upper) }.map(bound_from_c);

        let bounds = LazyBounds::new(lower, upper);
        *out = handle_to_c(bounds);
        Ok(())
    })
}

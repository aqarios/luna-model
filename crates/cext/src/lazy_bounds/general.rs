use luna_model::{core::bounds::Concretize, core::prelude::LazyBounds};

use crate::{
    LmBounds, LmLazyBounds, LmStatus, LmVtype,
    error::{CapiError, guard},
    utils::vtype_from_c,
    utils::{handle_to_c, out_from_c, ref_from_c},
};

/// Resolves optional caller-supplied bounds against the variable type defaults.
///
/// Missing endpoints fall back to the type-specific defaults. Integer
/// bounds are additionally checked for integral endpoints.
///
/// # Safety
/// `lazy_bounds` must be a valid lazy-bounds handle created by this library.
/// `out` must point to writable `LmBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_concretize(
    lazy_bounds: *const LmLazyBounds,
    vtype: LmVtype,
    out: *mut *mut LmBounds,
) -> LmStatus {
    guard(|| {
        let bounds = unsafe { (lazy_bounds as *const LazyBounds).as_ref() }.cloned();
        let out = unsafe { out_from_c(out) }?;
        let b = bounds
            .concretize(&vtype_from_c(vtype))
            .map_err(CapiError::errored)?;
        *out = handle_to_c(b);
        Ok(())
    })
}

/// Clones a lazy-bounds into a new, independent handle written to `out`.
///
/// # Safety
/// `lazy_bounds` must be a valid lazy-bounds handle created by this library.
/// `out` must point to writable `LmLazyBounds *` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_clone(
    lazy_bounds: *const LmLazyBounds,
    out: *mut *mut LmLazyBounds,
) -> LmStatus {
    guard(|| {
        let lazy_bounds = unsafe { ref_from_c(lazy_bounds as *const LazyBounds, "lazy_bounds") }?;
        let out = unsafe { out_from_c(out) }?;
        *out = handle_to_c(*lazy_bounds);
        Ok(())
    })
}

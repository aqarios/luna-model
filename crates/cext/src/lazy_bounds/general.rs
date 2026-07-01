use luna_model::{core::bounds::Concretize, core::prelude::LazyBounds};

use crate::{
    LmBounds, LmLazyBounds, LmStatus, LmVtype,
    error::{CapiError, guard},
    utils::vtype_from_c,
};

/// Resolves optional caller-supplied bounds against the variable type defaults.
///
/// Missing endpoints fall back to the type-specific defaults. Integer
/// bounds are additionally checked for integral endpoints.
///
/// # Safety
/// `lazy_bounds` must be a valid lazy-bounds handle created by this library.
/// `out` must point to writable `LmBound` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_concretize(
    lazy_bounds: *const LmLazyBounds,
    vtype: LmVtype,
    out: *mut *mut LmBounds,
) -> LmStatus {
    guard(|| {
        let bounds = unsafe { (lazy_bounds as *const LazyBounds).as_ref() }.cloned();
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        let b = bounds
            .concretize(&vtype_from_c(vtype))
            .map_err(|e| CapiError::new(LmStatus::Errored, e.to_string()))?;
        *out = Box::into_raw(Box::new(b)) as *mut LmBounds;
        Ok(())
    })
}

/// Clones a lazy-bounds into a new, independent handle written to `out`.
///
/// # Safety
/// `lazy_bounds` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_lazy_bounds_clone(
    lazy_bounds: *const LmLazyBounds,
    out: *mut *mut LmLazyBounds,
) -> LmStatus {
    guard(|| {
        let lazy_bounds = unsafe { (lazy_bounds as *const LazyBounds).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("lazy_bounds"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(lazy_bounds.clone())) as *mut LmLazyBounds;
        Ok(())
    })
}

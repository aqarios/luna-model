use luna_model::core::prelude::Bounds;

use crate::{
    LmBounds, LmStatus,
    error::{CapiError, guard},
};

/// Clones a bounds into a new, independent handle written to `out`.
///
/// # Safety
/// `bounds` must be a valid handle and `out` must point to writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bounds_clone(
    bounds: *const LmBounds,
    out: *mut *mut LmBounds,
) -> LmStatus {
    guard(|| {
        let bounds = unsafe { (bounds as *const Bounds).as_ref() }
            .ok_or_else(|| CapiError::null_pointer("bounds"))?;
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = Box::into_raw(Box::new(bounds.clone())) as *mut LmBounds;
        Ok(())
    })
}

use crate::{
    LmBound, LmStatus,
    error::{CapiError, guard},
    utils::bound_from_c,
};

/// Returns if the bound is bounded.
///
/// # Safety
/// `bound` must be a valid bound handle created by this library.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bound_is_bounded(bound: *const LmBound, out: *mut bool) -> LmStatus {
    guard(|| {
        let bound = bound_from_c(
            unsafe { bound.as_ref().cloned() }.ok_or_else(|| CapiError::null_pointer("bound"))?,
        );
        let out = unsafe { out.as_mut() }.ok_or_else(|| CapiError::null_pointer("out"))?;
        *out = bound.is_bounded();
        Ok(())
    })
}

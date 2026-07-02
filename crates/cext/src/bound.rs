use crate::utils::{out_from_c, ref_from_c};
use crate::{LmBound, LmStatus, error::guard, utils::bound_from_c};

/// Returns whether the bound is finite (`Bounded`) rather than `Unbounded`.
///
/// # Safety
/// `bound` must point to a readable `LmBound`.
/// `out` must point to writable `bool` storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_bound_is_bounded(bound: *const LmBound, out: *mut bool) -> LmStatus {
    guard(|| {
        let bound = bound_from_c(*unsafe { ref_from_c(bound, "bound") }?);
        let out = unsafe { out_from_c(out) }?;
        *out = bound.is_bounded();
        Ok(())
    })
}

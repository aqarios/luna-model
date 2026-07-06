mod access;

use luna_model::core::Solution;

use crate::{LmSampleView, utils::drop_from_c};

/// Owned backing for an [`LmSampleView`]: the borrowed solution and its row index.
pub struct SampleViewHandle {
    pub sol: *const Solution,
    pub index: usize,
}

/// Frees a sample-view handle. Passing null is a no-op.
///
/// # Safety
/// `view` must originate from [`lm_sol_sample`] and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_sample_view_free(view: *mut LmSampleView) {
    unsafe { drop_from_c(view as *mut SampleViewHandle) }
}

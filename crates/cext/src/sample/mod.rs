mod access;

pub use access::{
    lm_sample_view_assignment, lm_sample_view_assignment_idx, lm_sample_view_index,
    lm_sample_view_variable_names,
};

use luna_model::core::Solution;

use crate::{LmSampleView, utils::drop_from_c};

/// Owned backing for an [`LmSampleView`].
///
/// Keeping a solution snapshot makes the opaque view independent from the
/// lifetime and subsequent mutation of the solution handle that created it.
pub struct SampleViewHandle {
    pub sol: Solution,
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

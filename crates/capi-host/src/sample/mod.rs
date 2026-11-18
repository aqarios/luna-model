mod access;

pub use access::{
    lm_sample_view_assignment, lm_sample_view_assignment_idx, lm_sample_view_index,
    lm_sample_view_variable_names,
};

use lunamodel_capi_handle::Shared;

use crate::{LmSampleView, LmSol, utils::drop_from_c};

/// Owned backing for an [`LmSampleView`].
///
/// Retaining the shared solution keeps the view live and independent from the
/// lifetime of the particular solution handle that created it.
pub struct SampleViewHandle {
    pub sol: Shared<LmSol>,
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

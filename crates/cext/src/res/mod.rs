mod access;

pub use access::{
    lm_result_view_constraints, lm_result_view_counts, lm_result_view_feasible,
    lm_result_view_obj_value, lm_result_view_raw_energy, lm_result_view_sample,
    lm_result_view_variable_bounds,
};

use luna_model::core::Solution;

use crate::{LmResultView, utils::drop_from_c};

/// Owned backing for an [`LmResultView`].
///
/// Keeping a solution snapshot makes the opaque view independent from the
/// lifetime and subsequent mutation of the solution handle that created it.
pub struct ResultViewHandle {
    pub sol: Solution,
    pub index: usize,
}

/// Frees a result-view handle. Passing null is a no-op.
///
/// # Safety
/// `view` must originate from [`lm_sol_result`] and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_result_view_free(view: *mut LmResultView) {
    unsafe { drop_from_c(view as *mut ResultViewHandle) }
}

mod access;

use luna_model::core::Solution;

use crate::{LmResultView, utils::drop_from_c};

/// Owned backing for an [`LmResultView`]: the borrowed solution and its row index.
pub struct ResultViewHandle {
    pub sol: *const Solution,
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

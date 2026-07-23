mod access;

use lunamodel_core::solution::result::ResultView;
use lunamodel_capi_handle::Shared;

use crate::{LmResultView, LmSol, utils::drop_from_c};

pub use access::{
    lm_result_view_constraints, lm_result_view_counts, lm_result_view_feasible,
    lm_result_view_obj_value, lm_result_view_raw_energy, lm_result_view_sample,
    lm_result_view_variable_bounds,
};

/// Owned backing for an [`LmResultView`].
///
/// Retaining the shared solution keeps the view live and independent from the
/// lifetime of the particular solution handle that created it.
pub struct ResultViewHandle {
    pub sol: Shared<LmSol>,
    pub index: usize,
}

impl ResultViewHandle {
    pub fn read<R>(&self, f: impl FnOnce(ResultView<'_>) -> R) -> R {
        let solution = self.sol.read();
        f(ResultView::new(&solution, self.index))
    }
}

/// Frees a result-view handle. Passing null is a no-op.
///
/// # Safety
/// `view` must originate from [`lm_sol_result`] and must not be used or freed again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lm_result_view_free(view: *mut LmResultView) {
    unsafe { drop_from_c(view as *mut ResultViewHandle) }
}

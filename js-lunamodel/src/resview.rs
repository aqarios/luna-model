use std::collections::HashMap;

use lunamodel::core::Solution as CoreSolution;
use lunamodel::core::solution::result::ResultView;
use napi::bindgen_prelude::{Error, Result, Status};
use napi_derive::napi;

/// Provides access to solution information including the sample,
/// objective value, feasibility, and constraint satisfaction, counts,
/// raw energy, and comparison capabilities.
#[napi(js_name = "ResultView")]
pub struct JsResultView {
    inner: CoreSolution,
    idx: usize,
}

impl JsResultView {
    pub fn new(inner: CoreSolution, idx: usize) -> Self {
        Self { inner, idx }
    }
}

#[napi]
impl JsResultView {
    /// Get the number of times this result was observed.
    #[napi(getter)]
    pub fn counts(&self) -> Result<u32> {
        let counts = ResultView::new(&self.inner, self.idx).counts();
        u32::try_from(counts).map_err(|_| {
            Error::new(
                Status::GenericFailure,
                format!("counts {counts} is too large to return as a JavaScript number"),
            )
        })
    }

    /// Get the objective function value.
    #[napi(getter)]
    pub fn obj_value(&self) -> Option<f64> {
        ResultView::new(&self.inner, self.idx).obj_value()
    }

    /// Get the raw energy from the solver.
    #[napi(getter)]
    pub fn raw_energy(&self) -> Option<f64> {
        ResultView::new(&self.inner, self.idx).raw_energy()
    }

    /// Get constraint satisfaction status.
    #[napi(getter)]
    pub fn constraints(&self) -> Option<HashMap<String, bool>> {
        ResultView::new(&self.inner, self.idx).constraints()
    }

    /// Get variable bound satisfaction status.
    #[napi(getter)]
    pub fn variable_bounds(&self) -> Option<HashMap<String, bool>> {
        ResultView::new(&self.inner, self.idx).variable_bounds()
    }

    /// Get feasibility status.
    #[napi(getter)]
    pub fn feasible(&self) -> Option<bool> {
        ResultView::new(&self.inner, self.idx).feasible()
    }
}

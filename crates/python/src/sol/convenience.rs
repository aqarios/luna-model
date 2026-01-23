use lunamodel_core::ValueSource;
use lunamodel_unwind::unwindable;
use pyo3::{PyResult, pymethods};

use super::PySolution;
use crate::sol::result::PyResultView;
use crate::unwind::unwind;

#[unwindable]
#[pymethods]
impl PySolution {
    pub fn best(&self) -> Option<Vec<PyResultView>> {
        self.s.read_arc().best().map(|vs| {
            vs.iter()
                .map(|v| PyResultView::new(self.clone(), v.idx))
                .collect()
        })
    }

    pub fn cvar(&self, alpha: f64, value_toggle: ValueSource) -> PyResult<f64> {
        Ok(self.s.read_arc().cvar(alpha, Some(value_toggle))?)
    }

    pub fn temperature_weighted(&self, beta: f64, value_toggle: ValueSource) -> PyResult<f64> {
        Ok(self
            .s
            .read_arc()
            .temperature_weighted(beta, Some(value_toggle))?)
    }

    pub fn expectation_value(&self, value_toggle: ValueSource) -> PyResult<f64> {
        Ok(self.s.read_arc().expectation_value(Some(value_toggle))?)
    }

    pub fn feasibility_ratio(&self) -> PyResult<f64> {
        Ok(self.s.read_arc().feasibility_ratio()?)
    }

    pub fn highest_constraint_violation(&self) -> PyResult<Option<String>> {
        Ok(self.s.read_arc().highest_constraint_violations()?)
    }
}

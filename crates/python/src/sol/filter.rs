use lunamodel_core::Solution;
use lunamodel_error::LunaModelError;
use pyo3::exceptions::PyTypeError;
use pyo3::types::PyAnyMethods;
use pyo3::{Bound, PyAny, PyResult, pymethods};

use crate::sol::result::PyResultView;

use super::PySolution;

#[pymethods]
impl PySolution {
    pub fn filter<'py>(&self, f: Bound<'py, PyAny>) -> PyResult<PySolution> {
        if !f.is_callable() {
            return Err(PyTypeError::new_err("The parameter 'f' must be a callable"));
        }
        Ok(self
            .s
            .read_arc()
            .filter(|view| {
                let pyview: PyResultView = PyResultView::new(view.sol.clone().into(), view.idx);
                let r: bool = f
                    .call1((pyview,))
                    .map_err(|e| LunaModelError::Computation(e.to_string().into()))?
                    .extract::<bool>()
                    .map_err(|e| LunaModelError::Computation(e.to_string().into()))?;
                Ok(r)
            })?
            .into())
    }

    pub fn filter_feasible(&self) -> PyResult<PySolution> {
        let slf: &Solution = &self.s.read_arc();
        if let Some(feasibles) = &slf.feasible {
            Ok(slf.filter_by_mask(feasibles)?.into())
        } else {
            Err(LunaModelError::Computation(
                "no feasible information on solution, evaluate first.".into(),
            )
            .into())
        }
    }
}

use std::collections::HashMap;

use lunamodel_core::solution::result::ResultView;
use lunamodel_unwind::*;
use pyo3::{PyResult, pyclass, pymethods};

use super::super::sample::PySampleView;
use crate::sol::PySolution;

#[pyclass]
#[derive(Debug)]
pub struct PyResultView {
    pub sol: PySolution,
    pub idx: usize,
}

impl PyResultView {
    pub fn new(sol: PySolution, idx: usize) -> Self {
        Self { sol, idx }
    }
}

#[unwindable]
#[pymethods]
impl PyResultView {
    #[getter]
    fn counts(&self) -> usize {
        ResultView::new(&self.sol.s.read_arc(), self.idx).counts()
    }

    #[getter]
    fn obj_value(&self) -> Option<f64> {
        ResultView::new(&self.sol.s.read_arc(), self.idx).obj_value()
    }

    #[getter]
    fn raw_energy(&self) -> Option<f64> {
        ResultView::new(&self.sol.s.read_arc(), self.idx).raw_energy()
    }

    #[getter]
    fn constraints(&self) -> Option<HashMap<String, bool>> {
        // TODO: can we make the hashbrown::HashMap returnable?
        ResultView::new(&self.sol.s.read_arc(), self.idx)
            .constraints()
            .map(|c| c.into_iter().collect())
    }

    #[getter]
    fn variable_bounds(&self) -> Option<HashMap<String, bool>> {
        // TODO: can we make the hashbrown::HashMap returnable?
        ResultView::new(&self.sol.s.read_arc(), self.idx)
            .variable_bounds()
            .map(|v| v.into_iter().collect())
    }

    #[getter]
    fn feasible(&self) -> Option<bool> {
        ResultView::new(&self.sol.s.read_arc(), self.idx).feasible()
    }

    #[getter]
    fn sample(&self) -> PySampleView {
        PySampleView::new(self.sol.clone(), self.idx)
    }

    fn __str__(&self) -> PyResult<String> {
        PySampleView::new(self.sol.clone(), self.idx).__str__()
    }
}

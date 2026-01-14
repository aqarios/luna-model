use pyo3::{pyclass, pymethods};

use super::super::sample::PySampleView;
use crate::sol::PySolution;

#[pyclass]
pub struct PyResultView {
    pub sol: PySolution,
    pub idx: usize,
}

impl PyResultView {
    pub fn new(sol: PySolution, idx: usize) -> Self {
        Self { sol, idx }
    }
}

#[pymethods]
impl PyResultView {
    #[getter]
    fn sample(&self) -> PySampleView {
        PySampleView::new(self.sol.clone(), self.idx)
    }
}

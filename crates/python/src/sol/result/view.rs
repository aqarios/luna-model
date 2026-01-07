use pyo3::pyclass;

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

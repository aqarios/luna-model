use pyo3::pyclass;

use crate::sol::PySolution;

#[pyclass]
pub struct PySampleView {
    pub sol: PySolution,
    pub idx: usize,
}

impl PySampleView {
    pub fn new(sol: PySolution, idx: usize) -> Self {
        Self { sol, idx }
    }
}


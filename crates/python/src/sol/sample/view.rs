use std::collections::HashMap;

use pyo3::{pyclass, pymethods};

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

#[pymethods]
impl PySampleView {
    fn to_dict(&self) -> HashMap<String, f64> {
        self.sol
            .s
            .read_arc()
            .samples
            .iter()
            .map(|(varname, col)| (varname.clone(), col[self.idx]))
            .collect()
    }
}

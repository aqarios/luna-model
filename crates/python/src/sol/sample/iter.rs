use lunamodel_core::Solution;
use pyo3::{PyRef, PyRefMut, pyclass, pymethods};

use crate::sol::PySolution;

use super::view::PySampleView;

#[pyclass]
pub struct PySampleIterator {
    sol: PySolution,
    idx: usize,
}

impl PySampleIterator {
    pub fn new(sol: PySolution) -> Self {
        Self { sol, idx: 0 }
    }
}

#[pymethods]
impl PySampleIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PySampleView> {
        let binding: &Solution = &slf.sol.s.read_arc();
        let res = binding.result(slf.idx);
        slf.idx += 1;
        res.map(|_| PySampleView::new(slf.sol.clone(), slf.idx))
    }
}

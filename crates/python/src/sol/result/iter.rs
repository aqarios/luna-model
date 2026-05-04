//! Iterators over Python result views.

use lunamodel_core::Solution;
use lunamodel_unwind::*;
use pyo3::{PyRef, PyRefMut, pyclass, pymethods};

use super::view::PyResultView;
use crate::sol::PySolution;

#[pyclass]
pub struct PyResultIterator {
    sol: PySolution,
    idx: usize,
}

impl PyResultIterator {
    pub fn new(sol: PySolution) -> Self {
        Self { sol, idx: 0 }
    }
}

#[unwindable]
#[pymethods]
impl PyResultIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyResultView> {
        let binding: &Solution = &slf.sol.s.read_arc();
        let res = binding.result(slf.idx);
        let out = res.map(|_| PyResultView::new(slf.sol.clone(), slf.idx));
        slf.idx += 1;
        out
    }
}

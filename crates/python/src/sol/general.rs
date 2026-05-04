//! Core Python protocol helpers for `Solution`.

use lunamodel_unwind::*;
use pyo3::{
    Py, PyAny, PyRef, PyResult, Python,
    exceptions::{PyIndexError, PyTypeError, PyValueError},
    pymethods,
};

use super::PySolution;
use crate::{
    args::PySolArg,
    sol::result::{PyResultIterator, PyResultView},
};

#[unwindable]
#[pymethods]
impl PySolution {
    /// Compare two solutions for exact equality.
    fn __eq__(&self, other: PySolArg) -> bool {
        self.s.read_arc().eq(&other.s.read_arc())
    }

    /// Index into the solution by result/sample position.
    ///
    /// The binding intentionally supports only integer indexing here so Python
    /// callers get a clear distinction between result access and name-based
    /// value lookup methods.
    fn __getitem__(&self, py: Python, item: Py<PyAny>) -> PyResult<PyResultView> {
        if let Ok(res_idx) = item.extract::<isize>(py) {
            if res_idx < 0 {
                return Err(PyValueError::new_err(format!(
                    "Expected a non-negative number, received: {res_idx}"
                )))?;
            }
            match self.s.read_arc().result(res_idx as usize) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {res_idx} out of bounds"
                ))),
                Some(r) => Ok(PyResultView::new(self.clone(), r.idx)),
            }
        } else {
            Err(PyTypeError::new_err("unsupported type for indexing"))
        }
    }

    /// Iterate over result views in sample order.
    fn __iter__(slf: PyRef<'_, Self>) -> PyResultIterator {
        PyResultIterator::new(slf.clone())
    }

    /// Return the number of stored samples.
    fn __len__(&self) -> usize {
        self.s.read_arc().len()
    }
}

use crate::sol::result::{PyResultIterator, PyResultView};

use super::PySolution;
use pyo3::{
    Py, PyAny, PyRef, PyResult, Python,
    exceptions::{PyIndexError, PyTypeError, PyValueError},
    pymethods,
};

#[pymethods]
impl PySolution {
    fn __eq__(&self, other: &Self) -> bool {
        self.s.read_arc().eq(&other.s.read_arc())
    }

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

    fn __iter__(slf: PyRef<'_, Self>) -> PyResultIterator {
        PyResultIterator::new(slf.clone())
    }
}

use derive_more::{Deref, DerefMut};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};

#[derive(Deref, DerefMut, Clone, Copy, Debug)]
pub struct PyUsize(pub usize);

impl Into<usize> for PyUsize {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for PyUsize {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl<'py> FromPyObject<'py> for PyUsize {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let n: i128 = ob.extract()?;
        if n < 0 {
            Err(PyValueError::new_err(format!(
                "Expected a non-negative number, received: {n}"
            )))
        } else {
            Ok(Self(n as usize))
        }
    }
}

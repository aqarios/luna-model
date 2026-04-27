use derive_more::Deref;
use pyo3::{FromPyObject, PyErr, PyRef, exceptions::PyTypeError, types::PyAnyMethods};

use crate::PyBounds;

#[derive(Deref, Debug)]
pub struct PyBoundsArg(PyBounds);

impl From<PyBoundsArg> for PyBounds {
    fn from(val: PyBoundsArg) -> Self {
        val.0
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyBoundsArg {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        if let Ok(c) = obj.extract::<PyRef<'py, PyBounds>>() {
            return Ok(Self(c.clone()));
        }

        if let Ok(inner) = obj.getattr("_b")
            && let Ok(c) = inner.extract::<PyRef<'py, PyBounds>>()
        {
            return Ok(Self(c.clone()));
        }

        Err(PyTypeError::new_err("Expected (Py)Bounds"))
    }
}

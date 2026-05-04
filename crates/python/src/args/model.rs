//! Flexible Python argument extraction for model-like inputs.

use derive_more::Deref;
use pyo3::{FromPyObject, PyErr, PyRef, exceptions::PyTypeError, types::PyAnyMethods};

use crate::PyModel;

#[derive(Deref, Debug)]
pub struct PyModelArg(PyModel);

impl From<PyModelArg> for PyModel {
    fn from(val: PyModelArg) -> Self {
        val.0
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyModelArg {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        if let Ok(c) = obj.extract::<PyRef<'py, PyModel>>() {
            return Ok(Self(c.clone()));
        }

        if let Ok(inner) = obj.getattr("_m")
            && let Ok(c) = inner.extract::<PyRef<'py, PyModel>>()
        {
            return Ok(Self(c.clone()));
        }

        Err(PyTypeError::new_err("Expected (Py)Model"))
    }
}

use derive_more::Deref;
use pyo3::{FromPyObject, PyErr, PyRef, exceptions::PyTypeError, types::PyAnyMethods};

use crate::PyConstraint;

#[derive(Deref, Debug)]
pub struct PyCArg(PyConstraint);

impl Into<PyConstraint> for PyCArg {
    fn into(self) -> PyConstraint {
        self.0
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyCArg {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        if let Ok(c) = obj.extract::<PyRef<'py, PyConstraint>>() {
            return Ok(Self(c.clone()));
        }

        if let Ok(inner) = obj.getattr("_c") {
            if let Ok(c) = inner.extract::<PyRef<'py, PyConstraint>>() {
                return Ok(Self(c.clone()));
            }
        }

        Err(PyTypeError::new_err("Expected (Py)Constraint"))
    }
}

use derive_more::Deref;
use pyo3::{FromPyObject, PyErr, PyRef, exceptions::PyTypeError, types::PyAnyMethods};

use crate::PyConstraintCollection;

#[derive(Deref, Debug)]
pub struct PyColArg(pub PyConstraintCollection);

impl From<PyColArg> for PyConstraintCollection {
    fn from(val: PyColArg) -> Self {
        val.0
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyColArg {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        if let Ok(c) = obj.extract::<PyRef<'py, PyConstraintCollection>>() {
            return Ok(Self(c.clone()));
        }

        if let Ok(inner) = obj.getattr("_cc")
            && let Ok(c) = inner.extract::<PyRef<'py, PyConstraintCollection>>()
        {
            return Ok(Self(c.clone()));
        }

        Err(PyTypeError::new_err("Expected (Py)ConstraintCollection"))
    }
}

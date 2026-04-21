use derive_more::Deref;
use pyo3::{FromPyObject, PyErr, PyRef, exceptions::PyTypeError, types::PyAnyMethods};

use crate::PySolution;

#[derive(Deref, Debug)]
pub struct PySolArg(PySolution);

impl Into<PySolution> for PySolArg {
    fn into(self) -> PySolution {
        self.0
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PySolArg {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        if let Ok(c) = obj.extract::<PyRef<'py, PySolution>>() {
            return Ok(Self(c.clone()));
        }

        if let Ok(inner) = obj.getattr("_s") {
            if let Ok(c) = inner.extract::<PyRef<'py, PySolution>>() {
                return Ok(Self(c.clone()));
            }
        }

        Err(PyTypeError::new_err("Expected (Py)Solution"))
    }
}

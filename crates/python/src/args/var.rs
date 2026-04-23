use derive_more::Deref;
use pyo3::{FromPyObject, PyErr, PyRef, exceptions::PyTypeError, types::PyAnyMethods};

use crate::PyVariable;

#[derive(Deref, Debug, Clone)]
pub struct PyVarArg(pub PyVariable);

impl From<PyVarArg> for PyVariable {
    fn from(val: PyVarArg) -> Self {
        val.0
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyVarArg {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        if let Ok(c) = obj.extract::<PyRef<'py, PyVariable>>() {
            return Ok(Self(c.clone()));
        }

        if let Ok(inner) = obj.getattr("_v")
            && let Ok(c) = inner.extract::<PyRef<'py, PyVariable>>() {
                return Ok(Self(c.clone()));
            }

        Err(PyTypeError::new_err("Expected (Py)Variable"))
    }
}

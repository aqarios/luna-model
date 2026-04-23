use derive_more::Deref;
use pyo3::{FromPyObject, PyErr, PyRef, exceptions::PyTypeError, types::PyAnyMethods};

use crate::PyModelSpecs;

#[derive(Deref, Debug)]
pub struct PyModelSpecsArg(PyModelSpecs);

impl From<PyModelSpecsArg> for PyModelSpecs {
    fn from(val: PyModelSpecsArg) -> Self {
        val.0
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyModelSpecsArg {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        if let Ok(c) = obj.extract::<PyRef<'py, PyModelSpecs>>() {
            return Ok(Self(c.clone()));
        }

        if let Ok(inner) = obj.getattr("_sp")
            && let Ok(c) = inner.extract::<PyRef<'py, PyModelSpecs>>() {
                return Ok(Self(c.clone()));
            }

        Err(PyTypeError::new_err("Expected (Py)ModelSpecs"))
    }
}

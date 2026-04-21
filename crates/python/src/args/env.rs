use derive_more::Deref;
use pyo3::{FromPyObject, PyErr, PyRef, exceptions::PyTypeError, types::PyAnyMethods};

use crate::{PyEnvironment, environment::get_active_env};

#[derive(Deref, Debug)]
pub struct PyEnvArg(PyEnvironment);

impl Into<PyEnvironment> for PyEnvArg {
    fn into(self) -> PyEnvironment {
        self.0
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyEnvArg {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        if let Ok(c) = obj.extract::<PyRef<'py, PyEnvironment>>() {
            return Ok(Self(c.clone()));
        }

        if let Ok(inner) = obj.getattr("_env") {
            if let Ok(c) = inner.extract::<PyRef<'py, PyEnvironment>>() {
                return Ok(Self(c.clone()));
            }
        }

        Err(PyTypeError::new_err("Expected (Py)Environment"))
    }
}

impl TryFrom<Option<PyEnvArg>> for PyEnvironment {
    type Error = PyErr;
    fn try_from(value: Option<PyEnvArg>) -> Result<Self, Self::Error> {
        match value {
            Some(env) => Ok(env.0.clone()),
            None => get_active_env(),
        }
    }
}

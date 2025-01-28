#[cfg(feature = "py")]
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "py")]
use pyo3::PyErr;
#[cfg(feature = "py")]
use pyo3::{create_exception, exceptions::PyException};
use std::fmt;
#[cfg(feature = "py")]
create_exception!(aq_models, VariableExistsException, PyException);

#[derive(Debug, Clone)]
pub struct VariableExistsError;

impl std::error::Error for VariableExistsError {}

impl fmt::Display for VariableExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Oh no!")
    }
}

impl std::convert::From<VariableExistsError> for PyErr {
    fn from(err: VariableExistsError) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}

use std::fmt;

#[cfg(feature = "py")]
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "py")]
use pyo3::PyErr;
#[cfg(feature = "py")]
use pyo3::{create_exception, exceptions::PyException};
#[cfg(feature = "py")]
create_exception!(aq_models, VariableExistsException, PyException);
#[cfg(feature = "py")]
create_exception!(aq_models, VariablesFromDifferentEnvsException, PyException);
#[cfg(feature = "py")]
create_exception!(aq_models, DifferentEnvsException, PyException);
#[cfg(feature = "py")]
create_exception!(aq_models, NoActiveEnvironmentFoundException, PyException);
#[cfg(feature = "py")]
create_exception!(aq_models, MultipleActiveEnvironmentsException, PyException);

#[derive(Debug, Clone)]
pub struct VariableExistsError;

impl std::error::Error for VariableExistsError {}

impl fmt::Display for VariableExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "variable already exists in environment")
    }
}

#[cfg(feature = "py")]
impl std::convert::From<VariableExistsError> for PyErr {
    fn from(err: VariableExistsError) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct VariablesFromDifferentEnvsError;

impl std::error::Error for VariablesFromDifferentEnvsError {}

impl fmt::Display for VariablesFromDifferentEnvsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "operation on two variables from differeent environments is not supported"
        )
    }
}

#[cfg(feature = "py")]
impl std::convert::From<VariablesFromDifferentEnvsError> for PyErr {
    fn from(err: VariablesFromDifferentEnvsError) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct DifferentEnvsError;

impl std::error::Error for DifferentEnvsError {}

impl fmt::Display for DifferentEnvsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "operation on two variables from differeent environments is not supported"
        )
    }
}

#[cfg(feature = "py")]
impl std::convert::From<DifferentEnvsError> for PyErr {
    fn from(err: DifferentEnvsError) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}

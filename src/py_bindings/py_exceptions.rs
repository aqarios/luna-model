use crate::core::expression::VariableOutOfRangeError;
use pyo3::{create_exception, exceptions::PyRuntimeError, PyErr};

create_exception!(aq_models, VariableOutOfRangeException, PyRuntimeError);

impl From<VariableOutOfRangeError> for PyErr {
    fn from(value: VariableOutOfRangeError) -> Self {
        VariableOutOfRangeException::new_err(value.to_string())
    }
}

use crate::core::exceptions::{
    DifferentEnvsError, VariableCreationError, VariablesFromDifferentEnvsError,
};
use crate::core::{exceptions::VariableExistsError, expression::VariableOutOfRangeError};
use crate::serialization::DecodeError;
use crate::translator::matrix_translator::ModelNotQuadraticError;
use pyo3::exceptions::PyException;
use pyo3::{create_exception, exceptions::PyRuntimeError, PyErr};
use std::convert::From;

create_exception!(aq_models, VariableOutOfRangeException, PyException);
create_exception!(aq_models, VariableExistsException, PyRuntimeError);
create_exception!(aq_models, VariablesFromDifferentEnvsException, PyException);
create_exception!(aq_models, DifferentEnvsException, PyException);
create_exception!(aq_models, NoActiveEnvironmentFoundException, PyException);
create_exception!(aq_models, MultipleActiveEnvironmentsException, PyException);

impl From<VariableOutOfRangeError> for PyErr {
    fn from(value: VariableOutOfRangeError) -> Self {
        VariableOutOfRangeException::new_err(value.to_string())
    }
}

impl From<VariableExistsError> for PyErr {
    fn from(err: VariableExistsError) -> PyErr {
        VariableExistsException::new_err(err.to_string())
    }
}

impl From<VariableCreationError> for PyErr {
    fn from(err: VariableCreationError) -> PyErr {
        VariableExistsException::new_err(err.to_string())
    }
}

impl From<VariablesFromDifferentEnvsError> for PyErr {
    fn from(err: VariablesFromDifferentEnvsError) -> PyErr {
        VariablesFromDifferentEnvsException::new_err(err.to_string())
    }
}

impl From<DifferentEnvsError> for PyErr {
    fn from(err: DifferentEnvsError) -> PyErr {
        DifferentEnvsException::new_err(err.to_string())
    }
}

impl From<DecodeError> for PyErr {
    fn from(err: DecodeError) -> PyErr {
        PyRuntimeError::new_err(err.to_string())
    }
}

impl From<ModelNotQuadraticError> for PyErr {
    fn from(err: ModelNotQuadraticError) -> Self {
        PyRuntimeError::new_err(err.to_string())
    }
}

use crate::core::expression::VariableOutOfRangeError;
use crate::errors::{
    DifferentEnvsError, MatrixTranslatorError, ModelNotQuadraticError, ModelNotUnconstrainedError,
    VariableCreationError, VariableExistsError, VariablesFromDifferentEnvsError,
};
use crate::serialization::DecodeError;
use pyo3::exceptions::PyException;
use pyo3::{create_exception, PyErr};
use std::convert::From;

create_exception!(aqmodels, VariableOutOfRangeException, PyException);
create_exception!(aqmodels, VariableExistsException, PyException);
create_exception!(aqmodels, VariablesFromDifferentEnvsException, PyException);
create_exception!(aqmodels, DifferentEnvsException, PyException);
create_exception!(aqmodels, NoActiveEnvironmentFoundException, PyException);
create_exception!(aqmodels, MultipleActiveEnvironmentsException, PyException);
create_exception!(aqmodels, DecodeException, PyException);
create_exception!(aqmodels, ModelNotQuadraticException, PyException);
create_exception!(aqmodels, ModelNotUnconstrainedException, PyException);

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
        DecodeException::new_err(err.to_string())
    }
}

impl From<ModelNotQuadraticError> for PyErr {
    fn from(err: ModelNotQuadraticError) -> Self {
        ModelNotQuadraticException::new_err(err.to_string())
    }
}

impl From<ModelNotUnconstrainedError> for PyErr {
    fn from(err: ModelNotUnconstrainedError) -> Self {
        ModelNotUnconstrainedException::new_err(err.to_string())
    }
}

impl From<MatrixTranslatorError> for PyErr {
    fn from(err: MatrixTranslatorError) -> Self {
        match err {
            MatrixTranslatorError::Constrained(err) => err.into(),
            MatrixTranslatorError::HigherOrder(err) => err.into(),
        }
    }
}

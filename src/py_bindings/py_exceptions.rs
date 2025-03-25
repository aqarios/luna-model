use crate::core::expression::VariableOutOfRangeErr;
use crate::errors::{
    DifferentEnvsErr, MatrixTranslatorErr, ModelNotQuadraticErr, ModelNotUnconstrainedErr,
    VariableCreationErr, VariableExistsErr, VariablesFromDifferentEnvsErr,
};
use crate::serialization::DecodeError as DecodeErr;
use pyo3::exceptions::PyException;
use pyo3::{create_exception, PyErr};
use std::convert::From;

create_exception!(aqmodels.exceptions, VariableOutOfRangeError, PyException);
create_exception!(aqmodels.exceptions, VariableExistsError, PyException);
create_exception!(
    aqmodels.exceptions,
    VariablesFromDifferentEnvsError,
    PyException
);
create_exception!(aqmodels.exceptions, DifferentEnvsError, PyException);
create_exception!(
    aqmodels.exceptions,
    NoActiveEnvironmentFoundError,
    PyException
);
create_exception!(
    aqmodels.exceptions,
    MultipleActiveEnvironmentsError,
    PyException
);
create_exception!(aqmodels.exceptions, DecodeError, PyException);
create_exception!(aqmodels.exceptions, ModelNotQuadraticError, PyException);
create_exception!(aqmodels.exceptions, ModelNotUnconstrainedError, PyException);

impl From<VariableOutOfRangeErr> for PyErr {
    fn from(value: VariableOutOfRangeErr) -> Self {
        VariableOutOfRangeError::new_err(value.to_string())
    }
}

impl From<VariableExistsErr> for PyErr {
    fn from(err: VariableExistsErr) -> PyErr {
        VariableExistsError::new_err(err.to_string())
    }
}

impl From<VariableCreationErr> for PyErr {
    fn from(err: VariableCreationErr) -> PyErr {
        VariableExistsError::new_err(err.to_string())
    }
}

impl From<VariablesFromDifferentEnvsErr> for PyErr {
    fn from(err: VariablesFromDifferentEnvsErr) -> PyErr {
        VariablesFromDifferentEnvsError::new_err(err.to_string())
    }
}

impl From<DifferentEnvsErr> for PyErr {
    fn from(err: DifferentEnvsErr) -> PyErr {
        DifferentEnvsError::new_err(err.to_string())
    }
}

impl From<DecodeErr> for PyErr {
    fn from(err: DecodeErr) -> PyErr {
        DecodeError::new_err(err.to_string())
    }
}

impl From<ModelNotQuadraticErr> for PyErr {
    fn from(err: ModelNotQuadraticErr) -> Self {
        ModelNotQuadraticError::new_err(err.to_string())
    }
}

impl From<ModelNotUnconstrainedErr> for PyErr {
    fn from(err: ModelNotUnconstrainedErr) -> Self {
        ModelNotUnconstrainedError::new_err(err.to_string())
    }
}

impl From<MatrixTranslatorErr> for PyErr {
    fn from(err: MatrixTranslatorErr) -> Self {
        match err {
            MatrixTranslatorErr::Constrained(err) => err.into(),
            MatrixTranslatorErr::HigherOrder(err) => err.into(),
        }
    }
}

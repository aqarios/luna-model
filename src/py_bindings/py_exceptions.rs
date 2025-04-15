use crate::core::expression::VariableOutOfRangeErr;
use crate::errors::{
    BqmTranslatorErr, DifferentEnvsErr, IndexOutOfBoundsErr, MatrixTranslatorErr,
    ModelNotQuadraticErr, ModelNotUnconstrainedErr, ModelVtypeErr, SolutionCreatorErr,
    TranslationErr, VariableCreationErr, VariableExistsErr, VariableNotExistingErr,
    VariablesFromDifferentEnvsErr,
};
use crate::serialization::DecodeError as DecodeErr;
use pyo3::exceptions::{PyException, PyIndexError};
use pyo3::{create_exception, PyErr};
use std::convert::From;

create_exception!(aqmodels.errors, VariableOutOfRangeError, PyException);
create_exception!(aqmodels.errors, VariableExistsError, PyException);
create_exception!(aqmodels.errors, VariableNotExistingError, PyException);
create_exception!(
    aqmodels.errors,
    VariablesFromDifferentEnvsError,
    PyException
);
create_exception!(aqmodels.errors, DifferentEnvsError, PyException);
create_exception!(aqmodels.errors, NoActiveEnvironmentFoundError, PyException);
create_exception!(
    aqmodels.errors,
    MultipleActiveEnvironmentsError,
    PyException
);
create_exception!(aqmodels.errors, DecodeError, PyException);
create_exception!(aqmodels.errors, ModelNotQuadraticError, PyException);
create_exception!(aqmodels.errors, ModelNotUnconstrainedError, PyException);
create_exception!(aqmodels.errors, ModelVtypeError, PyException);
create_exception!(aqmodels.errors, SolutionCreationError, PyException);
create_exception!(aqmodels.errors, TranslationError, PyException);

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

impl From<VariableNotExistingErr> for PyErr {
    fn from(err: VariableNotExistingErr) -> PyErr {
        VariableNotExistingError::new_err(err.to_string())
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

impl From<ModelVtypeErr> for PyErr {
    fn from(err: ModelVtypeErr) -> Self {
        ModelVtypeError::new_err(err.to_string())
    }
}

impl From<IndexOutOfBoundsErr> for PyErr {
    fn from(value: IndexOutOfBoundsErr) -> Self {
        PyIndexError::new_err(value.to_string())
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

impl From<BqmTranslatorErr> for PyErr {
    fn from(err: BqmTranslatorErr) -> Self {
        match err {
            BqmTranslatorErr::Constrained(err) => err.into(),
            BqmTranslatorErr::HigherOrder(err) => err.into(),
            BqmTranslatorErr::Vtype(err) => err.into(),
        }
    }
}

impl From<SolutionCreatorErr> for PyErr {
    fn from(value: SolutionCreatorErr) -> Self {
        SolutionCreationError::new_err(value.to_string())
    }
}

impl From<TranslationErr> for PyErr {
    fn from(value: TranslationErr) -> Self {
        TranslationError::new_err(value.to_string())
    }
}

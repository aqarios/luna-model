use crate::core::expression::VariableOutOfRangeErr;
use crate::errors::{
    BqmTranslatorErr, ComputationErr, DifferentEnvsErr, DuplicateConstraintNameErr, EvaluationErr,
    IllegalConstraintNameErr, IndexOutOfBoundsErr, MatrixTranslatorErr, ModelNotQuadraticErr,
    ModelNotUnconstrainedErr, ModelSenseNotMinimizeErr, ModelVtypeErr, SampleIncompatibleVtypeErr,
    SampleIncorrectLengthErr, SampleUnexpectedVariableErr, SolutionCreationErr, TranslationErr,
    VariableCreationErr, VariableNotExistingErr, VariablesFromDifferentEnvsErr,
};
use crate::serialization::DecodeError as DecodeErr;
use crate::transformations::errors::CompilationError as CompilationErr;
use pyo3::exceptions::{PyException, PyIndexError, PyRuntimeError, PyTypeError};
use pyo3::{create_exception, PyErr};
use std::convert::From;

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    ComputationError,
    PyException,
    "Raised when an error occurred in an internal computation."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    ComputationError,
    PyException,
    "Raised when an error occurred in an internal computation."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    DuplicateConstraintNameError,
    PyException,
    "Raised when a duplicate constraint name is used."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    DuplicateConstraintNameError,
    PyException,
    "Raised when a duplicate constraint name is used."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    VariableOutOfRangeError,
    PyException,
    "Raised when a variable referenced in an expression is out of bounds for the environment.

This error typically occurs when querying coefficients (linear, quadratic,
or higher-order) from an `Expression` using a `Variable` whose index does not
exist in the environment's internal registry.

This may happen if:
    - A variable is used from a different environment
    - A variable was removed or never registered properly
    - A raw index or tuple refers to a non-existent variable ID"
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    VariableOutOfRangeError,
    PyException,
    "Raised when a variable referenced in an expression is out of bounds for the environment.

This error typically occurs when querying coefficients (linear, quadratic,
or higher-order) from an `Expression` using a `Variable` whose index does not
exist in the environment's internal registry.

This may happen if:
    - A variable is used from a different environment
    - A variable was removed or never registered properly
    - A raw index or tuple refers to a non-existent variable ID"
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    VariableExistsError,
    PyException,
    "Raised when trying to create a variable with a name that already exists.

Variable names must be unique within an `Environment`. Attempting to redefine
a variable with the same name will raise this exception."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    VariableExistsError,
    PyException,
    "Raised when trying to create a variable with a name that already exists.

Variable names must be unique within an `Environment`. Attempting to redefine
a variable with the same name will raise this exception."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    VariableNotExistingError,
    PyException,
    "Raised when trying to get a variable with a name that does not exist."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    VariableNotExistingError,
    PyException,
    "Raised when trying to get a variable with a name that does not exist."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    VariableCreationError,
    PyException,
    "Raised when an error occurs during the creation of a variable.

For example, binary and spin variables cannot be created with bounds."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    VariableCreationError,
    PyException,
    "Raised when an error occurs during the creation of a variable.

For example, binary and spin variables cannot be created with bounds."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    VariablesFromDifferentEnvsError,
    PyException,
    "Raised when multiple variables from different environments are used together.

All variables in an expression or constraint must belong to the same
`Environment`. Mixing across environments is disallowed to ensure consistency."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    VariablesFromDifferentEnvsError,
    PyException,
    "Raised when multiple variables from different environments are used together.

All variables in an expression or constraint must belong to the same
`Environment`. Mixing across environments is disallowed to ensure consistency."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    DifferentEnvsError,
    PyException,
    "Raised when two incompatible environments are passed to a model or operation.

Unlike `VariablesFromDifferentEnvsError`, this error may occur at the model level
or in structural operations that require consistency across multiple environments."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    DifferentEnvsError,
    PyException,
    "Raised when two incompatible environments are passed to a model or operation.

Unlike `VariablesFromDifferentEnvsError`, this error may occur at the model level
or in structural operations that require consistency across multiple environments."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    NoActiveEnvironmentFoundError,
    PyException,
    "Raised when a variable or expression is created without an active environment context.

This typically happens when not using `with Environment(): ...` and no environment
was explicitly provided."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    NoActiveEnvironmentFoundError,
    PyException,
    "Raised when a variable or expression is created without an active environment context.

This typically happens when not using `with Environment(): ...` and no environment
was explicitly provided."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    MultipleActiveEnvironmentsError,
    PyException,
    "Raised when multiple environments are active simultaneously.

This is a logic error, since `aqmodels` only supports one active environment
at a time. This is enforced to maintain clarity and safety."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    MultipleActiveEnvironmentsError,
    PyException,
    "Raised when multiple environments are active simultaneously.

This is a logic error, since `aqmodels` only supports one active environment
at a time. This is enforced to maintain clarity and safety."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    DecodeError,
    PyException,
    "Raised when decoding or deserialization of binary data fails.

This can occur if the encoded data is corrupted, incompatible, or not generated
by `.encode()`."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    DecodeError,
    PyException,
    "Raised when decoding or deserialization of binary data fails.

This can occur if the encoded data is corrupted, incompatible, or not generated
by `.encode()`."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    IllegalConstraintNameError,
    PyException,
    "Raised when a constraint is tried to be created with an illegal name."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    IllegalConstraintNameError,
    PyException,
    "Raised when a constraint is tried to be created with an illegal name."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    TranslationError,
    PyException,
    "Raised when an error occurred during translation."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    TranslationError,
    PyException,
    "Raised when an error occurred during translation."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    ModelNotQuadraticError,
    TranslationError,
    "Raised when a model is expected to be quadratic but contains higher-order terms.

Some solvers or transformations require the model to have at most quadratic
expressions. This error signals that unsupported terms were detected."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    ModelNotQuadraticError,
    TranslationError,
    "Raised when a model is expected to be quadratic but contains higher-order terms.

Some solvers or transformations require the model to have at most quadratic
expressions. This error signals that unsupported terms were detected."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    ModelNotUnconstrainedError,
    TranslationError,
    "Raised when an operation requires an unconstrained model, but constraints are present.

Some solution methods may only work on unconstrained models, such as when
transforming a symbolic model to a low-level format."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    ModelNotUnconstrainedError,
    TranslationError,
    "Raised when an operation requires an unconstrained model, but constraints are present.

Some solution methods may only work on unconstrained models, such as when
transforming a symbolic model to a low-level format."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    ModelSenseNotMinimizeError,
    TranslationError,
    "Raised when an operation requires a model with minimization sense, but has maximization sense.

Some model formats only work with minimization sense. In this case, consider
setting the sense to `minimize` before the transformation, and multiplying the
objective by `-1` if necessary."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    ModelSenseNotMinimizeError,
    TranslationError,
    "Raised when an operation requires a model with minimization sense, but has maximization sense.

Some model formats only work with minimization sense. In this case, consider
setting the sense to `minimize` before the transformation, and multiplying the
objective by `-1` if necessary."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    ModelVtypeError,
    TranslationError,
    "Raised when an operation has certain constraints on a model's variable types that are violated.

Some solution methods may only work on models where all variables have the same
type, or where only certain variable types are permitted."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    ModelVtypeError,
    TranslationError,
    "Raised when an operation has certain constraints on a model's variable types that are violated.

Some solution methods may only work on models where all variables have the same
type, or where only certain variable types are permitted."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    VariableNamesError,
    TranslationError,
    "Raised when the QuboTranslator tries to create a model from a QUBO matrix, but the provided variable names are invalid.

If variable names are provided to the QuboTranslator, they have to be unique, and the number of names has to match the number of variables in the QUBO matrix."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    VariableNamesError,
    TranslationError,
    "Raised when the QuboTranslator tries to create a model from a QUBO matrix, but the provided variable names are invalid.

If variable names are provided to the QuboTranslator, they have to be unique, and the number of names has to match the number of variables in the QUBO matrix."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    EvaluationError,
    PyException,
    "Raised when an error occured during evaluation of a model."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    EvaluationError,
    PyException,
    "Raised when an error occured during evaluation of a model."
);

#[cfg(not(feature = "lq"))]
create_exception!(aqmodels._core.errors, SolutionTranslationError, PyException);
#[cfg(feature = "lq")]
create_exception!(luna_quantum._core.errors, SolutionTranslationError, PyException);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    SampleIncorrectLengthError,
    SolutionTranslationError,
    "Raised when a sample length is different from the number of model variables.

When an external solution format is translated to an AqSolution, the number of
variable assignments in the solution's sample has to exactly match the number of
variables in the model environment that is passed to the translator."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    SampleIncorrectLengthError,
    SolutionTranslationError,
    "Raised when a sample length is different from the number of model variables.

When an external solution format is translated to an AqSolution, the number of
variable assignments in the solution's sample has to exactly match the number of
variables in the model environment that is passed to the translator."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    SampleUnexpectedVariableError,
    SolutionTranslationError,
    "Raised when a sample contains a variable with a name that is not present in the environment.

When a sample is translated to an AqResult, the currently active environment has to
contain the same variables as the sample."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    SampleUnexpectedVariableError,
    SolutionTranslationError,
    "Raised when a sample contains a variable with a name that is not present in the environment.

When a sample is translated to an AqResult, the currently active environment has to
contain the same variables as the sample."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    SampleIncompatibleVtypeError,
    SolutionTranslationError,
    "Raised when a sample's assignments have variable types incompatible with the model's variable types.

When an external solution format is translated to an AqSolution, the variable
assignments are tried to be converted into the model's corresponding variable type.
This may fail when the assignment types are incompatible.

Note that conversions with precision loss or truncation are admitted, but
conversions of variables outside the permitted range will fail."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    SampleIncompatibleVtypeError,
    SolutionTranslationError,
    "Raised when a sample's assignments have variable types incompatible with the model's variable types.

When an external solution format is translated to an AqSolution, the variable
assignments are tried to be converted into the model's corresponding variable type.
This may fail when the assignment types are incompatible.

Note that conversions with precision loss or truncation are admitted, but
conversions of variables outside the permitted range will fail."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    StartCannotBeInferredError,
    PyTypeError,
    "To be raised when the start value in the quicksum cannot be inferred."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    StartCannotBeInferredError,
    PyTypeError,
    "To be raised when the start value in the quicksum cannot be inferred."
);

#[cfg(not(feature = "lq"))]
create_exception!(
    aqmodels._core.errors,
    CompilationError,
    PyRuntimeError,
    "Raised when an error occured during compilation of a model in the PassManager."
);
#[cfg(feature = "lq")]
create_exception!(
    luna_quantum._core.errors,
    CompilationError,
    PyRuntimeError,
    "Raised when an error occured during compilation of a model in the PassManager."
);

impl From<VariableOutOfRangeErr> for PyErr {
    fn from(value: VariableOutOfRangeErr) -> Self {
        VariableOutOfRangeError::new_err(value.to_string())
    }
}

impl From<VariableNotExistingErr> for PyErr {
    fn from(err: VariableNotExistingErr) -> PyErr {
        VariableNotExistingError::new_err(err.to_string())
    }
}

impl From<VariableCreationErr> for PyErr {
    fn from(err: VariableCreationErr) -> PyErr {
        match err {
            VariableCreationErr::VariableExists(_) => VariableExistsError::new_err(err.to_string()),
            VariableCreationErr::InvalidBounds(_) => {
                VariableCreationError::new_err(err.to_string())
            }
            VariableCreationErr::VarName(_) => VariableNamesError::new_err(err.to_string()),
        }
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

impl From<ModelSenseNotMinimizeErr> for PyErr {
    fn from(err: ModelSenseNotMinimizeErr) -> Self {
        ModelSenseNotMinimizeError::new_err(err.to_string())
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
            MatrixTranslatorErr::Maximize(err) => err.into(),
            MatrixTranslatorErr::Vtype(err) => err.into(),
            MatrixTranslatorErr::VarCreation(err) => err.into(),
        }
    }
}

impl From<BqmTranslatorErr> for PyErr {
    fn from(err: BqmTranslatorErr) -> Self {
        match err {
            BqmTranslatorErr::Constrained(err) => err.into(),
            BqmTranslatorErr::HigherOrder(err) => err.into(),
            BqmTranslatorErr::Maximize(err) => err.into(),
            BqmTranslatorErr::Vtype(err) => err.into(),
        }
    }
}

impl From<SolutionCreationErr> for PyErr {
    fn from(value: SolutionCreationErr) -> Self {
        match value {
            SolutionCreationErr::SampleIncorrectLength(err) => err.into(),
            SolutionCreationErr::SampleUnexpectedVariable(err) => err.into(),
            SolutionCreationErr::SampleIncompatibleVtype(err) => err.into(),
        }
    }
}

impl From<SampleIncorrectLengthErr> for PyErr {
    fn from(value: SampleIncorrectLengthErr) -> Self {
        SampleIncorrectLengthError::new_err(value.to_string())
    }
}

impl From<SampleUnexpectedVariableErr> for PyErr {
    fn from(value: SampleUnexpectedVariableErr) -> Self {
        SampleUnexpectedVariableError::new_err(value.to_string())
    }
}

impl From<SampleIncompatibleVtypeErr> for PyErr {
    fn from(value: SampleIncompatibleVtypeErr) -> Self {
        SampleIncompatibleVtypeError::new_err(value.to_string())
    }
}

impl From<TranslationErr> for PyErr {
    fn from(value: TranslationErr) -> Self {
        TranslationError::new_err(value.to_string())
    }
}

impl From<IllegalConstraintNameErr> for PyErr {
    fn from(value: IllegalConstraintNameErr) -> Self {
        IllegalConstraintNameError::new_err(value.to_string())
    }
}

impl From<ComputationErr> for PyErr {
    fn from(value: ComputationErr) -> Self {
        ComputationError::new_err(value.to_string())
    }
}

impl From<EvaluationErr> for PyErr {
    fn from(value: EvaluationErr) -> Self {
        EvaluationError::new_err(format!("{value}"))
    }
}

impl From<DuplicateConstraintNameErr> for PyErr {
    fn from(value: DuplicateConstraintNameErr) -> Self {
        DuplicateConstraintNameError::new_err(format!("{value}"))
    }
}

impl From<CompilationErr> for PyErr {
    fn from(value: CompilationErr) -> Self {
        CompilationError::new_err(value.to_string())
    }
}

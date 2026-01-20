// the entire module is feature bound to `py`.
use pyo3::{
    PyErr, create_exception,
    exceptions::{PyException, PyIndexError},
};

use crate::LunaModelError as Lme;

create_exception!(
    builtins.errors,
    PyLunaModelError,
    PyException,
    "The based LunaModel error type."
);

create_exception!(
    builtins.errors,
    PyUnsupportedOperationError,
    PyLunaModelError,
    "Raised when an operation is used on an unsupported type."
);

create_exception!(
    builtins.errors,
    PyCompressionError,
    PyLunaModelError,
    "Raised when an error occurred during the compression step of encoding."
);

create_exception!(
    builtins.errors,
    PyInternalPanicError,
    PyLunaModelError,
    "Raised when an internal and unrecoverable error occurred."
);

create_exception!(
    builtins.errors,
    PyComputationError,
    PyLunaModelError,
    "Raised when an error occurred in an internal computation."
);

create_exception!(
    builtins.errors,
    PyDuplicateConstraintNameError,
    PyLunaModelError,
    "Raised when a duplicate constraint name is used."
);

create_exception!(
    builtins.errors,
    PyVariableOutOfRangeError,
    PyLunaModelError,
    "Raised when a variable referenced in an expression is out of bounds for the environment.

This error typically occurs when querying coefficients (linear, quadratic,
or higher-order) from an `Expression` using a `Variable` whose index does not
exist in the environment's internal registry.

This may happen if:
    - A variable is used from a different environment
    - A variable was removed or never registered properly
    - A raw index or tuple refers to a non-existent variable ID"
);

create_exception!(
    builtins.errors,
    PyVariableExistsError,
    PyLunaModelError,
    "Raised when trying to create a variable with a name that already exists.

Variable names must be unique within an `Environment`. Attempting to redefine
a variable with the same name will raise this exception."
);

create_exception!(
    builtins.errors,
    PyVariableNotExistingError,
    PyLunaModelError,
    "Raised when trying to get a variable with a name that does not exist."
);

create_exception!(
    builtins.errors,
    PyVariableCreationError,
    PyLunaModelError,
    "Raised when an error occurs during the creation of a variable.

For example, binary and spin variables cannot be created with bounds."
);

create_exception!(
    builtins.errors,
    PyVariablesFromDifferentEnvsError,
    PyLunaModelError,
    "Raised when multiple variables from different environments are used together.

All variables in an expression or constraint must belong to the same
`Environment`. Mixing across environments is disallowed to ensure consistency."
);

create_exception!(
    builtins.errors,
    PyDifferentEnvsError,
    PyLunaModelError,
    "Raised when two incompatible environments are passed to a model or operation.

Unlike `VariablesFromDifferentEnvsError`, this error may occur at the model level
or in structural operations that require consistency across multiple environments."
);

create_exception!(
    builtins.errors,
    PyNoActiveEnvironmentFoundError,
    PyLunaModelError,
    "Raised when a variable or expression is created without an active environment context.

This typically happens when not using `with Environment(): ...` and no environment
was explicitly provided."
);

create_exception!(
    builtins.errors,
    PyMultipleActiveEnvironmentsError,
    PyLunaModelError,
    "Raised when multiple environments are active simultaneously.

This is a logic error, since `luna_model` only supports one active environment
at a time. This is enforced to maintain clarity and safety."
);

create_exception!(
    builtins.errors,
    PyDecodeError,
    PyLunaModelError,
    "Raised when decoding or deserialization of binary data fails.

This can occur if the encoded data is corrupted, incompatible, or not generated
by `.encode()`."
);

create_exception!(
    builtins.errors,
    PyIllegalConstraintNameError,
    PyLunaModelError,
    "Raised when a constraint is tried to be created with an illegal name."
);

create_exception!(
    builtins.errors,
    PyTranslationError,
    PyLunaModelError,
    "Raised when an error occurred during translation."
);

create_exception!(
    builtins.errors,
    PyModelNotQuadraticError,
    PyTranslationError,
    "Raised when a model is expected to be quadratic but contains higher-order terms.

Some solvers or transformations require the model to have at most quadratic
expressions. This error signals that unsupported terms were detected."
);

create_exception!(
    builtins.errors,
    PyModelNotUnconstrainedError,
    PyTranslationError,
    "Raised when an operation requires an unconstrained model, but constraints are present.

Some solution methods may only work on unconstrained models, such as when
transforming a symbolic model to a low-level format."
);

create_exception!(
    builtins.errors,
    PyModelSenseNotMinimizeError,
    PyTranslationError,
    "Raised when an operation requires a model with minimization sense, but has maximization sense.

Some model formats only work with minimization sense. In this case, consider
setting the sense to `minimize` before the transformation, and multiplying the
objective by `-1` if necessary."
);

create_exception!(
    builtins.errors,
    PyModelVtypeError,
    PyTranslationError,
    "Raised when an operation has certain constraints on a model's variable types that are violated.

Some solution methods may only work on models where all variables have the same
type, or where only certain variable types are permitted."
);

create_exception!(
    builtins.errors,
    PyVariableNamesError,
    PyTranslationError,
    "Raised when the QuboTranslator tries to create a model from a QUBO matrix, but the provided variable names are invalid.

If variable names are provided to the QuboTranslator, they have to be unique, and the number of names has to match the number of variables in the QUBO matrix."
);

create_exception!(
    builtins.errors,
    PyEvaluationError,
    PyLunaModelError,
    "Raised when an error occured during evaluation of a model."
);

create_exception!(
    builtins.errors,
    PySolutionTranslationError,
    PyLunaModelError
);

create_exception!(
    builtins.errors,
    PySampleIncorrectLengthError,
    PySolutionTranslationError,
    "Raised when a sample length is different from the number of model variables.

When an external solution format is translated to an LunaModel Solution, the number of
variable assignments in the solution's sample has to exactly match the number of
variables in the model environment that is passed to the translator."
);

create_exception!(
    builtins.errors,
    PySampleUnexpectedVariableError,
    PySolutionTranslationError,
    "Raised when a sample contains a variable with a name that is not present in the environment.

When a sample is translated to an LunaModel Result, the currently active environment has to
contain the same variables as the sample."
);

create_exception!(
    builtins.errors,
    PySampleIncompatibleVtypeError,
    PySolutionTranslationError,
    "Raised when a sample's assignments have variable types incompatible with the model's variable types.

When an external solution format is translated to a LunaModel Solution, the variable
assignments are tried to be converted into the model's corresponding variable type.
This may fail when the assignment types are incompatible.

Note that conversions with precision loss or truncation are admitted, but
conversions of variables outside the permitted range will fail."
);

create_exception!(
    builtins.errors,
    PyStartCannotBeInferredError,
    PyLunaModelError,
    "To be raised when the start value in the quicksum cannot be inferred."
);

create_exception!(
    builtins.errors,
    PySampleColCreationError,
    PyLunaModelError,
    "Raised when an error occured during creation of a sample column."
);

create_exception!(
    builtins.errors,
    PyNoConstraintForKeyError,
    PyLunaModelError,
    "Raised getting a constraint from the constraints that does not exist."
);

impl From<Lme> for PyErr {
    fn from(lme: Lme) -> Self {
        let err = match lme {
            Lme::DifferentEnvironments => PyDifferentEnvsError::new_err,
            Lme::VariableExists(_) => PyVariableExistsError::new_err,
            Lme::VariableNotExisting(_) => PyVariableNotExistingError::new_err,
            Lme::VariableNameInvalid(_) => PyVariableNamesError::new_err,
            Lme::ConstraintNameInvalid(_) => PyIllegalConstraintNameError::new_err,
            Lme::InvalidBounds(_) => PyVariableCreationError::new_err,
            Lme::InvalidInversion(_) => PyUnsupportedOperationError::new_err,
            Lme::Compression(_) => PyCompressionError::new_err,
            Lme::Decoding(_) => PyDecodeError::new_err,
            Lme::UnsupportedOperation(_) => PyUnsupportedOperationError::new_err,
            Lme::Dtype(_) => PySampleColCreationError::new_err,
            Lme::Formatter(_) | Lme::Internal(_) => PyLunaModelError::new_err,
            Lme::Computation(_) => PyComputationError::new_err,
            Lme::NoConstraintForKey(_) => PyNoConstraintForKeyError::new_err,
            Lme::DuplicateConstraintName(_) => PyDuplicateConstraintNameError::new_err,
            Lme::ModelNotQuadratic => PyModelNotQuadraticError::new_err,
            Lme::ModelNotUnconstrained => PyModelNotUnconstrainedError::new_err,
            Lme::Vtype(_) => PyModelVtypeError::new_err,
            Lme::Translation(_) => PyTranslationError::new_err,
            Lme::IndexOutOfBounds(_) => PyIndexError::new_err,
            Lme::ModelSenseNotMinimize => PyModelSenseNotMinimizeError::new_err,
            Lme::Evaluation(_) => PyEvaluationError::new_err,
            Lme::SampleIncorrectLength(_) => PySampleIncorrectLengthError::new_err,
            Lme::SampleUnexpectedVariable(_) => PySampleUnexpectedVariableError::new_err,
            Lme::SampleIncompatibleVtype => PySampleIncompatibleVtypeError::new_err,
            Lme::VariableNames(_) => PyVariableNamesError::new_err,
        };
        err(lme.to_string())
    }
}

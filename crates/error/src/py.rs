//! Python exception mapping for [`crate::LunaModelError`].
//!
//! This module is only compiled when the `py` feature is enabled. The exception
//! hierarchy mirrors the Rust-side error taxonomy closely enough that Python
//! callers can still handle specific categories without having to parse strings.
// the entire module is feature bound to `py`.
use pyo3::{
    PyErr, PyResult, Python, create_exception,
    exceptions::{PyBaseException, PyException, PyIndexError},
    prelude::*,
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

create_exception!(
    builtins.errors,
    PyRandomSamplingError,
    PyLunaModelError,
    "Raised when an error occured during random sampling."
);

create_exception!(
    builtins.errors,
    PyTransformationError,
    PyLunaModelError,
    "Raised when an error occured during a transformation."
);

create_exception!(
    builtins.errors,
    PyTransformationPassError,
    PyTransformationError,
    "Raised when an error occured during execution of a transformation pass."
);

create_exception!(
    builtins.errors,
    PyAnalysisPassError,
    PyTransformationError,
    "Raised when an error occured during execution of an analysis pass."
);

create_exception!(
    builtins.errors,
    PyIfElsePassError,
    PyTransformationError,
    "Raised when an error occured during execution of an analysis pass."
);

create_exception!(
    builtins.errors,
    PyMetaAnalysisPassError,
    PyTransformationError,
    "Raised when an error occured during execution of a meta analysis pass."
);

create_exception!(
    builtins.errors,
    PyTransformError,
    PyTransformationError,
    "Raised when an error occured during compilation."
);

create_exception!(
    builtins.errors,
    PyModelInfeasibleError,
    PyTransformError,
    "Raised when the model is found to be infeasible."
);

create_exception!(
    builtins.errors,
    PyInvalidToleranceError,
    PyLunaModelError,
    "Raised when an invalid tolerance is specified."
);

/// The constructor of the Python exception luna-model raises for an error of kind `tag`
/// ([`Lme::kind_tag`]).
///
/// This is the single kind -> exception mapping: native errors are raised through it (see the
/// `From<Lme> for PyErr` impl below), and C API extensions reach it through
/// `luna_model.errors._error_for_kind`, so an error raised by a plugin is the same Python class
/// as the native one. Kinds without a dedicated class (plugin-defined kinds, coarse ABI fallback
/// tags) map to the base `LunaModelError`.
pub fn new_err_for_kind(tag: &str) -> fn(String) -> PyErr {
    match tag {
        "different_environments" => PyDifferentEnvsError::new_err,
        "variable_exists" => PyVariableExistsError::new_err,
        "variable_not_existing" => PyVariableNotExistingError::new_err,
        "variable_name_invalid" | "variable_names" => PyVariableNamesError::new_err,
        "constraint_name_invalid" => PyIllegalConstraintNameError::new_err,
        "invalid_bounds" | "inverted_binary_creation" => PyVariableCreationError::new_err,
        "invalid_inversion" | "unsupported_operation" => PyUnsupportedOperationError::new_err,
        "compression" => PyCompressionError::new_err,
        "decoding" => PyDecodeError::new_err,
        "dtype" => PySampleColCreationError::new_err,
        "computation" => PyComputationError::new_err,
        "no_constraint_for_key" => PyNoConstraintForKeyError::new_err,
        "duplicate_constraint_name" => PyDuplicateConstraintNameError::new_err,
        "model_not_quadratic" => PyModelNotQuadraticError::new_err,
        "model_not_unconstrained" => PyModelNotUnconstrainedError::new_err,
        "vtype" => PyModelVtypeError::new_err,
        "translation" => PyTranslationError::new_err,
        "index_out_of_bounds" => PyIndexError::new_err,
        "model_sense_not_minimize" => PyModelSenseNotMinimizeError::new_err,
        "evaluation" => PyEvaluationError::new_err,
        "sample_incorrect_length" => PySampleIncorrectLengthError::new_err,
        "sample_unexpected_variable" => PySampleUnexpectedVariableError::new_err,
        "sample_incompatible_vtype" => PySampleIncompatibleVtypeError::new_err,
        "transformation_pass" => PyTransformationPassError::new_err,
        "analysis_pass" => PyAnalysisPassError::new_err,
        "if_else_pass" => PyIfElsePassError::new_err,
        "meta_analysis_pass" => PyMetaAnalysisPassError::new_err,
        "transformation" => PyTransformError::new_err,
        "random_sampling" => PyRandomSamplingError::new_err,
        "invalid_tolerance" => PyInvalidToleranceError::new_err,
        "infeasible" => PyModelInfeasibleError::new_err,
        // "formatter", "internal", "external", and any tag luna-model does not define.
        _ => PyLunaModelError::new_err,
    }
}

/// `luna_model.errors._error_for_kind(kind, message)`: the exception (not raised) luna-model
/// raises for an error of `kind`, see [`new_err_for_kind`].
#[pyfunction]
#[pyo3(name = "_error_for_kind")]
pub fn error_for_kind(
    py: Python<'_>,
    kind: &str,
    message: String,
) -> PyResult<Py<PyBaseException>> {
    Ok(new_err_for_kind(kind)(message).into_value(py))
}

impl From<Lme> for PyErr {
    fn from(lme: Lme) -> Self {
        let err = match lme {
            Lme::WithCause(e, cause) => {
                let pyerr: PyErr = (*e).into();
                // `cause` may itself be a `LunaModelError::WithCause` wrapping further context
                // frames (see `lunamodel_transpiler::error::with_cause`) before reaching an
                // actual `PyErr` — e.g. one `map_pyerr` built around a Python exception. Walk its
                // full `source()` chain rather than only checking the immediate cause, so the
                // original exception's identity survives as `__cause__` regardless of how many
                // context frames sit in between.
                if let Some(py_cause) = find_py_cause(cause.as_ref()) {
                    Python::attach(|py| pyerr.set_cause(py, Some(py_cause.clone_ref(py))));
                }
                return pyerr;
            }
            // Carries a plugin's own kind tag when a plugin pass failed; it still surfaces as a
            // transformation failure.
            Lme::Transformation { .. } => PyTransformError::new_err,
            ref other => new_err_for_kind(&other.kind_tag()),
        };
        err(lme.to_string())
    }
}

/// Finds a `PyErr` anywhere in `err`'s cause chain (including `err` itself), walking
/// `std::error::Error::source()`.
fn find_py_cause<'a>(mut err: &'a (dyn std::error::Error + 'static)) -> Option<&'a PyErr> {
    loop {
        if let Some(py_err) = err.downcast_ref::<PyErr>() {
            return Some(py_err);
        }
        err = err.source()?;
    }
}

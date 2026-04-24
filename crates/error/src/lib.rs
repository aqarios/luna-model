//! Shared error types for the LunaModel workspace.
//!
//! The goal of this crate is to keep error propagation consistent across the
//! Rust core, translators, serializers, transformation system, and optional
//! Python bindings. Most crates return [`LunaModelResult`] directly so that
//! higher-level integrations do not need to constantly remap domain errors.
use std::{
    error::Error,
    fmt::{Display, Formatter},
    ops::Deref,
};
mod froms;

#[cfg(feature = "py")]
pub mod py;

/// Cheap owned error message wrapper used by many [`LunaModelError`] variants.
#[derive(Debug, Clone)]
// pub struct ErrString(Cow<'static, str>);
pub struct ErrString(String);

impl<T> From<T> for ErrString
where
    // T: Into<Cow<'static, str>>,
    T: Into<String>,
{
    /// Converts any string-like payload into an owned error string.
    fn from(msg: T) -> Self {
        Self(msg.into())
    }
}

impl AsRef<str> for ErrString {
    /// Borrows the wrapped message as `&str`.
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ErrString {
    type Target = str;

    /// Dereferences to the wrapped string slice.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ErrString {
    /// Displays the wrapped message verbatim.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Common error enum used throughout the workspace.
#[derive(Debug, Clone)]
pub enum LunaModelError {
    /// Operands came from different environments.
    DifferentEnvironments,
    /// A variable name collided with an existing variable.
    VariableExists(ErrString),
    /// A requested variable was not present.
    VariableNotExisting(ErrString),
    /// A variable name failed validation.
    VariableNameInvalid(String, ErrString),
    /// A constraint name failed validation.
    ConstraintNameInvalid(ErrString),
    /// Bounds were invalid for the target variable type.
    InvalidBounds(ErrString),
    /// A variable inversion request was invalid.
    InvalidInversion(ErrString),
    /// Compression or decompression failed.
    Compression(ErrString),
    /// Serialized data could not be decoded.
    Decoding(ErrString),
    /// Formatting a developer/user-facing representation failed.
    Formatter(ErrString),
    /// The requested operation is not supported for the target object.
    UnsupportedOperation(ErrString),
    /// Internal invariant violation or unexpected state.
    Internal(ErrString),
    /// Data type conversion or interpretation failed.
    Dtype(ErrString),
    /// Numeric or symbolic computation failed.
    Computation(ErrString),
    /// A named constraint could not be found.
    NoConstraintForKey(ErrString),
    /// A duplicate constraint name was encountered.
    DuplicateConstraintName(ErrString),
    /// A model exceeded the quadratic degree limit.
    ModelNotQuadratic,
    /// A model unexpectedly contained constraints.
    ModelNotUnconstrained,
    /// A model unexpectedly had non-minimization sense.
    ModelSenseNotMinimize,
    /// Variable types violated a downstream requirement.
    Vtype(ErrString),
    /// Model or solution translation failed.
    Translation(ErrString),
    /// An index was out of bounds.
    IndexOutOfBounds(ErrString),
    /// Evaluation against a sample or assignment failed.
    Evaluation(ErrString),
    /// A sample had the wrong length.
    SampleIncorrectLength(ErrString),
    /// A sample referenced an unexpected variable.
    SampleUnexpectedVariable(ErrString),
    /// Sample assignments were incompatible with variable types.
    SampleIncompatibleVtype,
    /// Variable names failed a translator-specific requirement.
    VariableNames(ErrString),
    /// A transformation pass failed.
    TransformationPass(String, ErrString),
    /// An analysis pass failed.
    AnalysisPass(String, ErrString),
    /// An if/else control-flow pass failed.
    IfElsePass(ErrString),
    /// A meta-analysis pass failed.
    MetaAnalysisPass(String, ErrString),
    /// Compilation of a transformation/pipeline artifact failed.
    Compilation(ErrString),
    /// Random sampling failed.
    RandomSampling(ErrString),
    #[cfg(feature = "py")]
    /// Wraps a domain error together with a Python-side cause.
    WithCause(Box<LunaModelError>, py::PyErrW),
}

impl Error for LunaModelError {}

impl Display for LunaModelError {
    /// Formats the error in a human-readable developer-oriented form.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use LunaModelError::*;
        match self {
            VariableExists(msg) => write!(f, "variable exists: {}", msg),
            VariableNotExisting(msg) => write!(f, "variable does not exist: {}", msg),
            VariableNameInvalid(name, msg) => {
                write!(f, "variable name '{}' invalid: {}", name, msg)
            }
            ConstraintNameInvalid(msg) => write!(f, "constraint name invalid: {}", msg),
            InvalidBounds(msg) => write!(f, "invalid bounds: {}", msg),
            InvalidInversion(msg) => write!(f, "invalid inversion: {}", msg),
            Compression(msg) => write!(f, "compression failed: {}", msg),
            Decoding(msg) => write!(f, "decoding failed: {}", msg),
            Formatter(msg) => write!(f, "formatting failed: {}", msg),
            DifferentEnvironments => write!(f, "different environments encountered"),
            UnsupportedOperation(msg) => write!(f, "unsupported operation: {}", msg),
            Internal(msg) => write!(f, "internal LunaModel error: '{}'", msg),
            Dtype(msg) => write!(f, "invalid data type: {}", msg),
            Computation(msg) => write!(f, "error during computation: {}", msg),
            NoConstraintForKey(msg) => write!(f, "no constraint for key: {}", msg),
            DuplicateConstraintName(msg) => write!(f, "duplicate constraint name: {}", msg),
            ModelNotQuadratic => write!(f, "the model is not linear or quadratic"),
            ModelNotUnconstrained => write!(f, "the model is not unconstrained"),
            ModelSenseNotMinimize => write!(f, "the model's sense is not Minimize"),
            Vtype(msg) => write!(f, "unexpected Vtype: {}", msg),
            Translation(msg) => write!(f, "translation error: {}", msg),
            IndexOutOfBounds(msg) => write!(f, "index out of bounds: {}", msg),
            Evaluation(msg) => write!(f, "error in evaluation: {}", msg),
            SampleIncorrectLength(msg) => write!(f, "sample incorrect length: {}", msg),
            SampleUnexpectedVariable(msg) => {
                write!(f, "sample contains unexpected variable: {}", msg)
            }
            SampleIncompatibleVtype => {
                write!(f, "sample contains incompatible variable assignments")
            }
            VariableNames(msg) => write!(f, "{}", msg),
            TransformationPass(name, msg) => {
                write!(f, "error in Transformation pass '{}': {}", name, msg)
            }
            AnalysisPass(name, msg) => {
                write!(f, "error in Analysis pass '{}': {}", name, msg)
            }
            IfElsePass(msg) => write!(f, "error in IfElse pass: {}", msg),
            MetaAnalysisPass(name, msg) => {
                write!(f, "error in MetaAnalysis pass '{}': {}", name, msg)
            }
            Compilation(msg) => write!(f, "compilation error: {}", msg),
            RandomSampling(msg) => write!(f, "random sampling failed due to: {}", msg),
            #[cfg(feature = "py")]
            WithCause(err, _) => write!(f, "{}", err),
        }
    }
}

/// Workspace-wide result alias.
pub type LunaModelResult<T> = Result<T, LunaModelError>;

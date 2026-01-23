use std::{
    error::Error,
    fmt::{Display, Formatter},
    ops::Deref,
};
mod froms;

#[cfg(feature = "py")]
pub mod py;

#[derive(Debug, Clone)]
// pub struct ErrString(Cow<'static, str>);
pub struct ErrString(String);

impl<T> From<T> for ErrString
where
    // T: Into<Cow<'static, str>>,
    T: Into<String>,
{
    fn from(msg: T) -> Self {
        Self(msg.into())
    }
}

impl AsRef<str> for ErrString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ErrString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ErrString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub enum LunaModelError {
    DifferentEnvironments,
    VariableExists(ErrString),
    VariableNotExisting(ErrString),
    VariableNameInvalid(ErrString),
    ConstraintNameInvalid(ErrString),
    InvalidBounds(ErrString),
    InvalidInversion(ErrString),
    Compression(ErrString),
    Decoding(ErrString),
    Formatter(ErrString),
    UnsupportedOperation(ErrString),
    Internal(ErrString),
    Dtype(ErrString),
    Computation(ErrString),
    NoConstraintForKey(ErrString),
    DuplicateConstraintName(ErrString),
    ModelNotQuadratic,
    ModelNotUnconstrained,
    ModelSenseNotMinimize,
    Vtype(ErrString),
    Translation(ErrString),
    IndexOutOfBounds(ErrString),
    Evaluation(ErrString),
    SampleIncorrectLength(ErrString),
    SampleUnexpectedVariable(ErrString),
    SampleIncompatibleVtype,
    VariableNames(ErrString),
    TransformationPass(String, ErrString),
    AnalysisPass(String, ErrString),
    IfElsePass(ErrString),
    MetaAnalysisPass(String, ErrString),
    Compilation(ErrString),
}

impl Error for LunaModelError {}

impl Display for LunaModelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use LunaModelError::*;
        match self {
            VariableExists(msg) => write!(f, "variable exists: {}", msg),
            VariableNotExisting(msg) => write!(f, "variable does not exist: {}", msg),
            VariableNameInvalid(msg) => write!(f, "variable name invalid: {}", msg),
            ConstraintNameInvalid(msg) => write!(f, "constraint name invalid: {}", msg),
            InvalidBounds(msg) => write!(f, "invalid bounds: {}", msg),
            InvalidInversion(msg) => write!(f, "invalid inversion: {}", msg),
            Compression(msg) => write!(f, "compression failed: {}", msg),
            Decoding(msg) => write!(f, "decoding failed: {}", msg),
            Formatter(msg) => write!(f, "formatting failed: {}", msg),
            DifferentEnvironments => write!(f, "different environments encountered"),
            UnsupportedOperation(msg) => write!(f, "the operation '{}' is not supported", msg),
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
            MetaAnalysisPass(name, msg) => write!(f, "error in MetaAnalysis pass '{}': {}", name, msg),
            Compilation(msg) => write!(f, "compilation error: {}", msg),
        }
    }
}

pub type LunaModelResult<T> = Result<T, LunaModelError>;

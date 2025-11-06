use crate::core::Vtype;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    num::ParseIntError,
};

#[derive(Debug, Clone)]
pub struct ColumnCreationErr {
    msg: Option<String>,
}
impl ColumnCreationErr {
    pub fn default() -> Self {
        Self { msg: None }
    }
    pub fn new(msg: &str) -> Self {
        Self {
            msg: Some(String::from(msg)),
        }
    }
}
impl Error for ColumnCreationErr {}
impl Display for ColumnCreationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.msg {
            Some(msg) => write!(f, "error when creating sample column: {}", msg),
            None => write!(f, "error when creating sample column"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DuplicateConstraintNameErr(pub String);
impl Error for DuplicateConstraintNameErr {}
impl Display for DuplicateConstraintNameErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "duplicate constraint name used: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct UnsupportedNotOperationErr;

impl UnsupportedNotOperationErr {
    pub fn new(vtype: Vtype) -> UnsupportedOperationErr {
        UnsupportedOperationErr(String::from("not"), vtype.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct UnsupportedOperationErr(pub String, pub String);
impl Error for UnsupportedOperationErr {}
impl Display for UnsupportedOperationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The '{}' operation is not supported on variables with type: {}",
            self.0, self.1
        )
    }
}

#[derive(Debug, Clone)]
pub struct ComputationErr(pub String);
impl Error for ComputationErr {}
impl Display for ComputationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encountered error in computation: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct IllegalConstraintNameErr(pub String);
impl Error for IllegalConstraintNameErr {}
impl Display for IllegalConstraintNameErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Illegal constraint name: {}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableNotExistingErr;
impl Error for VariableNotExistingErr {}
impl Display for VariableNotExistingErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "variable does not exists in environment")
    }
}

#[derive(Debug, Clone)]
pub struct TranslationErr {
    msg: String,
}
impl TranslationErr {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl Error for TranslationErr {}
impl Display for TranslationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error encountered during translation: {}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub enum VariableCreationErr {
    VariableExists(String),
    InvalidBounds(Vtype),
    InvalidInversion(Vtype),
    VarName(String),
}
impl Error for VariableCreationErr {}
impl Display for VariableCreationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            VariableCreationErr::VariableExists(s) => {
                format!("variable '{s}' already exists")
            }
            VariableCreationErr::InvalidBounds(vtype) => {
                format!("bounds cannot be set for variable of type {vtype}.")
            }
            VariableCreationErr::InvalidInversion(vtype) => {
                format!("variables of type '{vtype}' cannot be inverted.")
            }
            VariableCreationErr::VarName(s) => s.clone(),
        };
        write!(f, "variable creation failed: {msg}")
    }
}

#[derive(Debug, Clone)]
pub struct VariablesFromDifferentEnvsErr;
impl Error for VariablesFromDifferentEnvsErr {}
impl Display for VariablesFromDifferentEnvsErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "operation on two variables from different environments is not supported"
        )
    }
}

#[derive(Debug, Clone)]
pub struct DifferentEnvsErr;
impl Error for DifferentEnvsErr {}
impl Display for DifferentEnvsErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "operation on two variables from different environments is not supported"
        )
    }
}

#[derive(Debug, Clone)]
pub struct ParseFromStringError(pub String);
impl Error for ParseFromStringError {}
impl Display for ParseFromStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not parse to string: {}", self.0)
    }
}
impl From<ParseIntError> for ParseFromStringError {
    fn from(err: ParseIntError) -> Self {
        ParseFromStringError(err.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ModelNotQuadraticErr;
impl Error for ModelNotQuadraticErr {}
impl Display for ModelNotQuadraticErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "the model is not linear or quadratic")
    }
}

#[derive(Debug, Clone)]
pub struct ModelSenseNotMinimizeErr;
impl Error for ModelSenseNotMinimizeErr {}
impl Display for ModelSenseNotMinimizeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "the model does not have the sense 'minimize'")
    }
}

#[derive(Debug, Clone)]
pub struct ModelNotUnconstrainedErr;
impl Error for ModelNotUnconstrainedErr {}
impl Display for ModelNotUnconstrainedErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "the model is not unconstrained")
    }
}

#[derive(Debug, Clone)]
pub struct ModelVtypeErr(pub String);
impl Error for ModelVtypeErr {}
impl Display for ModelVtypeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

#[derive(Debug, Clone)]
pub enum MatrixTranslatorErr {
    Constrained(ModelNotUnconstrainedErr),
    HigherOrder(ModelNotQuadraticErr),
    Maximize(ModelSenseNotMinimizeErr),
    Vtype(ModelVtypeErr),
    VarCreation(VariableCreationErr),
}
impl Error for MatrixTranslatorErr {}
impl Display for MatrixTranslatorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            MatrixTranslatorErr::Constrained(err) => err.fmt(f),
            MatrixTranslatorErr::HigherOrder(err) => err.fmt(f),
            MatrixTranslatorErr::Maximize(err) => err.fmt(f),
            MatrixTranslatorErr::Vtype(err) => err.fmt(f),
            MatrixTranslatorErr::VarCreation(err) => err.fmt(f),
        }
    }
}

impl From<ModelNotQuadraticErr> for MatrixTranslatorErr {
    fn from(value: ModelNotQuadraticErr) -> Self {
        Self::HigherOrder(value)
    }
}

impl From<ModelNotUnconstrainedErr> for MatrixTranslatorErr {
    fn from(value: ModelNotUnconstrainedErr) -> Self {
        Self::Constrained(value)
    }
}

impl From<ModelSenseNotMinimizeErr> for MatrixTranslatorErr {
    fn from(value: ModelSenseNotMinimizeErr) -> Self {
        Self::Maximize(value)
    }
}

impl From<ModelVtypeErr> for MatrixTranslatorErr {
    fn from(value: ModelVtypeErr) -> Self {
        Self::Vtype(value)
    }
}

impl From<VariableCreationErr> for MatrixTranslatorErr {
    fn from(value: VariableCreationErr) -> Self {
        Self::VarCreation(value)
    }
}

#[derive(Debug, Clone)]
pub enum BqmTranslatorErr {
    Constrained(ModelNotUnconstrainedErr),
    HigherOrder(ModelNotQuadraticErr),
    Maximize(ModelSenseNotMinimizeErr),
    Vtype(ModelVtypeErr),
}
impl Error for BqmTranslatorErr {}
impl Display for BqmTranslatorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            BqmTranslatorErr::Constrained(err) => err.fmt(f),
            BqmTranslatorErr::HigherOrder(err) => err.fmt(f),
            BqmTranslatorErr::Maximize(err) => err.fmt(f),
            BqmTranslatorErr::Vtype(err) => err.fmt(f),
        }
    }
}

impl From<ModelNotQuadraticErr> for BqmTranslatorErr {
    fn from(value: ModelNotQuadraticErr) -> Self {
        Self::HigherOrder(value)
    }
}

impl From<ModelNotUnconstrainedErr> for BqmTranslatorErr {
    fn from(value: ModelNotUnconstrainedErr) -> Self {
        Self::Constrained(value)
    }
}

impl From<ModelSenseNotMinimizeErr> for BqmTranslatorErr {
    fn from(value: ModelSenseNotMinimizeErr) -> Self {
        Self::Maximize(value)
    }
}

impl From<ModelVtypeErr> for BqmTranslatorErr {
    fn from(value: ModelVtypeErr) -> Self {
        Self::Vtype(value)
    }
}

#[derive(Debug, Clone)]
pub struct IndexOutOfBoundsErr {
    idx: usize,
    len: usize,
}
impl IndexOutOfBoundsErr {
    pub fn new(idx: usize, len: usize) -> Self {
        Self { idx, len }
    }
}
impl Error for IndexOutOfBoundsErr {}
impl Display for IndexOutOfBoundsErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "index '{}' out of bounds for constraints of len {}",
            self.idx, self.len
        )
    }
}

#[derive(Debug, Clone)]
pub struct SampleIncorrectLengthErr;

impl Error for SampleIncorrectLengthErr {}

impl Display for SampleIncorrectLengthErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sample length is different from the number of variables in the environment"
        )
    }
}

#[derive(Debug, Clone)]
pub struct SampleUnexpectedVariableErr {
    pub var_name: String,
}
impl SampleUnexpectedVariableErr {
    pub fn new(var_name: String) -> Self {
        Self { var_name }
    }
}

impl Error for SampleUnexpectedVariableErr {}

impl Display for SampleUnexpectedVariableErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "found unexpected variable in sample: '{}'",
            self.var_name
        )
    }
}

#[derive(Debug, Clone)]
pub struct SampleIncompatibleVtypeErr;

impl Error for SampleIncompatibleVtypeErr {}

impl Display for SampleIncompatibleVtypeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sample contains variable assignments incompatible with the model's variable types."
        )
    }
}

#[derive(Debug)]
pub enum SolutionCreationErr {
    SampleIncorrectLength(SampleIncorrectLengthErr),
    SampleUnexpectedVariable(SampleUnexpectedVariableErr),
    SampleIncompatibleVtype(SampleIncompatibleVtypeErr),
}

impl Error for SolutionCreationErr {}

impl Display for SolutionCreationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SolutionCreationErr::SampleIncorrectLength(err) => err.fmt(f),
            SolutionCreationErr::SampleUnexpectedVariable(err) => err.fmt(f),
            SolutionCreationErr::SampleIncompatibleVtype(err) => err.fmt(f),
        }
    }
}
impl From<SampleIncorrectLengthErr> for SolutionCreationErr {
    fn from(value: SampleIncorrectLengthErr) -> Self {
        SolutionCreationErr::SampleIncorrectLength(value)
    }
}
impl From<SampleIncompatibleVtypeErr> for SolutionCreationErr {
    fn from(value: SampleIncompatibleVtypeErr) -> Self {
        SolutionCreationErr::SampleIncompatibleVtype(value)
    }
}

impl From<IllegalConstraintNameErr> for TranslationErr {
    fn from(value: IllegalConstraintNameErr) -> Self {
        TranslationErr::new(value.0)
    }
}

impl From<DuplicateConstraintNameErr> for IllegalConstraintNameErr {
    fn from(value: DuplicateConstraintNameErr) -> Self {
        IllegalConstraintNameErr(value.to_string())
    }
}

#[derive(Debug)]
pub struct VariableOcc {
    pub only_in_sol: Option<Vec<String>>,
    pub only_in_env: Option<Vec<String>>,
}
impl VariableOcc {
    pub fn new(only_in_sol: Option<Vec<String>>, only_in_env: Option<Vec<String>>) -> Self {
        Self {
            only_in_sol,
            only_in_env,
        }
    }
}

impl VariableOcc {
    fn build_str(&self, what: &str) -> Result<String, std::fmt::Error> {
        let sep = ", ";
        let msg_sol = format!("{what} contains variables not present in model");
        let msg_env = "missing variables in solution present in model";
        match (&self.only_in_sol, &self.only_in_env) {
            (Some(s), None) => Ok(format!("{msg_sol}: '{}'", s.join(sep))),
            (None, Some(e)) => Ok(format!("{msg_env}: '{}'", e.join(sep))),
            (Some(s), Some(e)) => Ok(format!(
                "{msg_sol}: '{}', and {msg_env}: '{}'",
                s.join(sep),
                e.join(sep)
            )),
            (None, None) => Err(std::fmt::Error),
        }
    }
}

#[derive(Debug)]
pub enum GetConstraintErr {
    IndexOutOfBoundsErr(IndexOutOfBoundsErr),
    NoConstraintForKeyErr(String),
}
impl Error for GetConstraintErr {}
impl Display for GetConstraintErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::IndexOutOfBoundsErr(err) => err.to_string(),
            Self::NoConstraintForKeyErr(name) => {
                format!("no constraint for key: {name}").to_string()
            }
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub enum EvaluationErr {
    SolutionAndModelVariablesMismatch(VariableOcc),
    SampleAndModelVariablesMismatch(VariableOcc),
}
impl Error for EvaluationErr {}

impl Display for EvaluationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            EvaluationErr::SolutionAndModelVariablesMismatch(oc) => {
                format!(
                    "error occurred during solution evaluation: {}",
                    oc.build_str("solution")?
                )
            }
            EvaluationErr::SampleAndModelVariablesMismatch(oc) => {
                format!(
                    "error occurred during sample evaluation: {}",
                    oc.build_str("sample")?
                )
            }
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone)]
pub struct CompressionErr(pub String);
impl Error for CompressionErr {}
impl Display for CompressionErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "compression failed: '{}'", self.0)
    }
}

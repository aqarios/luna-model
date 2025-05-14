use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError,
};

#[derive(Debug, Clone)]
pub struct IllegalConstraintNameErr(pub String);
impl Error for IllegalConstraintNameErr {}
impl Display for IllegalConstraintNameErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Illegal constraint name: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct VariableExistsErr;
impl Error for VariableExistsErr {}
impl Display for VariableExistsErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "variable already exists in environment")
    }
}

#[derive(Debug, Clone)]
pub struct VariableNotExistingErr;
impl Error for VariableNotExistingErr {}
impl Display for VariableNotExistingErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "error encountered during translation: {}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub struct VariableCreationErr {
    msg: String,
}
impl VariableCreationErr {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl Error for VariableCreationErr {}
impl Display for VariableCreationErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "variable creation failed: {}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub struct VariablesFromDifferentEnvsErr;
impl Error for VariablesFromDifferentEnvsErr {}
impl Display for VariablesFromDifferentEnvsErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "the model is not linear or quadratic"
        )
    }
}

#[derive(Debug, Clone)]
pub struct ModelSenseNotMinimizeErr;
impl Error for ModelSenseNotMinimizeErr {}
impl Display for ModelSenseNotMinimizeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "the model does not have the sense 'minimize'"
        )
    }
}

#[derive(Debug, Clone)]
pub struct ModelNotUnconstrainedErr;
impl Error for ModelNotUnconstrainedErr {}
impl Display for ModelNotUnconstrainedErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "the model is not unconstrained")
    }
}

#[derive(Debug, Clone)]
pub struct ModelVtypeErr(pub String);
impl Error for ModelVtypeErr {}
impl Display for ModelVtypeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.0)
    }
}

#[derive(Debug, Clone)]
pub enum MatrixTranslatorErr {
    Constrained(ModelNotUnconstrainedErr),
    HigherOrder(ModelNotQuadraticErr),
    Maximize(ModelSenseNotMinimizeErr),
    Vtype(ModelVtypeErr),
}
impl Error for MatrixTranslatorErr {}
impl Display for MatrixTranslatorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            MatrixTranslatorErr::Constrained(err) => err.fmt(f),
            MatrixTranslatorErr::HigherOrder(err) => err.fmt(f),
            MatrixTranslatorErr::Maximize(err) => err.fmt(f),
            MatrixTranslatorErr::Vtype(err) => err.fmt(f),
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

#[derive(Debug, Clone)]
pub enum BqmTranslatorErr {
    Constrained(ModelNotUnconstrainedErr),
    HigherOrder(ModelNotQuadraticErr),
    Maximize(ModelSenseNotMinimizeErr),
    Vtype(ModelVtypeErr),
}
impl Error for BqmTranslatorErr {}
impl Display for BqmTranslatorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "index '{}' out of bounds for constraints of len {}",
            self.idx, self.len
        )
    }
}

#[derive(Debug, Clone)]
pub struct SampleIncorrectLengthError;

impl Error for SampleIncorrectLengthError {}

impl Display for SampleIncorrectLengthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Sample has a length different from the other samples in the solution"
        )
    }
}

#[derive(Debug, Clone)]
pub struct IncorrectVtypeError;

impl Error for IncorrectVtypeError {}

impl Display for IncorrectVtypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Found variable assignment of incorrect vtype.")
    }
}

#[derive(Debug)]
pub struct SolutionCreatorErr(pub String);

impl Error for SolutionCreatorErr {}

impl Display for SolutionCreatorErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}
impl From<SampleIncorrectLengthError> for SolutionCreatorErr {
    fn from(value: SampleIncorrectLengthError) -> Self {
        SolutionCreatorErr(value.to_string())
    }
}
impl From<IncorrectVtypeError> for SolutionCreatorErr {
    fn from(value: IncorrectVtypeError) -> Self {
        SolutionCreatorErr(value.to_string())
    }
}

impl From<IllegalConstraintNameErr> for TranslationErr {
    fn from(value: IllegalConstraintNameErr) -> Self {
        TranslationErr::new(value.0)
    }
}


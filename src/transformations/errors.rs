use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone)]
pub struct CompilationError(pub String);
impl Error for CompilationError {}
impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Compilation Error: {}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct TransformationPassError(pub String, pub String);
impl Error for TransformationPassError {}
impl Display for TransformationPassError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Error in transformation pass '{}': {}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisPassError(pub String, pub String);
impl Error for AnalysisPassError {}
impl Display for AnalysisPassError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Error in analysis pass '{}': {}", self.0, self.1)
    }
}

impl From<AnalysisPassError> for CompilationError {
    fn from(other: AnalysisPassError) -> CompilationError {
        CompilationError(format!("{}", other))
    }
}
impl From<TransformationPassError> for CompilationError {
    fn from(other: TransformationPassError) -> CompilationError {
        CompilationError(format!("{}", other))
    }
}

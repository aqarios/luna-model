use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableExistsError;
impl std::error::Error for VariableExistsError {}
impl fmt::Display for VariableExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "variable already exists in environment")
    }
}

#[derive(Debug, Clone)]
pub struct VariableCreationError {
    msg: String,
}
impl VariableCreationError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
impl std::error::Error for VariableCreationError {}
impl fmt::Display for VariableCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "variable creation failed: {}", self.msg)
    }
}

#[derive(Debug, Clone)]
pub struct VariablesFromDifferentEnvsError;
impl std::error::Error for VariablesFromDifferentEnvsError {}
impl fmt::Display for VariablesFromDifferentEnvsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "operation on two variables from differeent environments is not supported"
        )
    }
}

#[derive(Debug, Clone)]
pub struct DifferentEnvsError;
impl std::error::Error for DifferentEnvsError {}
impl fmt::Display for DifferentEnvsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "operation on two variables from differeent environments is not supported"
        )
    }
}

#[derive(Debug, Clone)]
pub struct ParseFromStringError(pub String);
impl std::error::Error for ParseFromStringError {}
impl fmt::Display for ParseFromStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "could not parse to string: {}", self.0)
    }
}

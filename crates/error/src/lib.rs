use std::{
    error::Error,
    fmt::{Display, Formatter},
    ops::Deref,
};
mod froms;

#[derive(Debug, Clone)]
// pub struct ErrString(Cow<'static, str>);
pub struct ErrString(String);

// impl ErrString {
//     pub const fn new_static(s: &'static str) -> Self {
//         Self(Cow::Borrowed(s))
//     }
// }

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
    VariableExists(ErrString),
    VariableNameInvalid(ErrString),
    ConstraintNameInvalid(ErrString),
    InvalidBounds(ErrString),
    InvalidInversion(ErrString),
    Compression(ErrString),
    Decoding(ErrString),
}

impl Error for LunaModelError {}

impl Display for LunaModelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use LunaModelError::*;
        match self {
            VariableExists(msg) => write!(f, "variable exists: {}", msg),
            VariableNameInvalid(msg) => write!(f, "variable name invalid: {}", msg),
            ConstraintNameInvalid(msg) => write!(f, "constraint name invalid: {}", msg),
            InvalidBounds(msg) => write!(f, "invalid bounds: {}", msg),
            InvalidInversion(msg) => write!(f, "invalid inversion: {}", msg),
            Compression(msg) => write!(f, "compression failed: {}", msg),
            Decoding(msg) => write!(f, "decoding failed: {}", msg),
        }
    }
}

pub type LunaModelResult<T> = Result<T, LunaModelError>;

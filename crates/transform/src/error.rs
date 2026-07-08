use std::{error::Error, fmt::Display};

pub type TransformResult<T> = Result<T, TransformError>;

#[derive(Debug)]
pub enum TransformError {
    Analysis {
        name: String,
        msg: String,
    },
    /// External error occured
    External {
        e: Box<dyn Error>,
    },
}
impl Error for TransformError {}

impl Display for TransformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Analysis { name, msg } => write!(f, "analysis '{name}' errored: {msg}"),
            Self::External { e } => write!(f, "external: {}", e.to_string()),
        }
    }
}

impl TransformError {
    pub fn external<E: Error + 'static>(err: E) -> Self {
        Self::External { e: Box::new(err) }
    }
}

impl<E: Error + 'static> From<E> for TransformError {
    fn from(value: E) -> Self {
        Self::External { e: Box::new(value) }
    }
}

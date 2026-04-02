use lunamodel_error::{ErrString, LunaModelError};

pub enum TransformationError {
    MissingAnalysis { name: &'static str },
    UnregisteredPass { name: String },
    ArtifactTypeMismatch {
        expected: &'static str,
        found: String,
    },
}

impl Into<ErrString> for TransformationError {
    fn into(self) -> ErrString {
        match self {
            Self::MissingAnalysis { name } => format!("missing analysis pass '{name}'").into(),
            Self::UnregisteredPass { name } => {
                format!("unregistered pass for backwards '{name}'").into()
            }
            Self::ArtifactTypeMismatch { expected, found } => {
                format!("artifact type mismatch: expected '{expected}', found '{found}'").into()
            }
        }
    }
}

impl From<TransformationError> for LunaModelError {
    fn from(value: TransformationError) -> Self {
        Self::Compilation(value.into())
    }
}

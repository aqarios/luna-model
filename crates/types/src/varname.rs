use std::fmt::Display;

use derive_more::{Deref, DerefMut};

const INV_PREFIX: &str = "~";

/// Variable name wrapper used across the workspace.
#[derive(Debug, Clone, Deref, DerefMut, Hash, PartialEq, Eq)]
pub struct VarName(pub String);

impl VarName {
    /// Returns the conventional name for the inverted companion of this variable.
    pub fn inverted(&self) -> Self {
        Self(format!("{INV_PREFIX}{}", self.0))
    }
}

impl From<&str> for VarName {
    /// Wraps a borrowed string as a variable name.
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<VarName> for String {
    /// Unwraps an owned variable name into its string.
    fn from(val: VarName) -> Self {
        val.0
    }
}

impl From<&VarName> for String {
    /// Clones the underlying string from a borrowed variable name.
    fn from(val: &VarName) -> Self {
        val.0.clone()
    }
}

impl Display for VarName {
    /// Displays the raw variable name string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

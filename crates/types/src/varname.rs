use std::fmt::Display;

use derive_more::{Deref, DerefMut};

const INV_PREFIX: &str = "~";

#[derive(Debug, Clone, Deref, DerefMut, Hash, PartialEq, Eq)]
pub struct VarName(pub String);

impl VarName {
    pub fn inverted(&self) -> Self {
        Self(format!("{INV_PREFIX}{}", self.0))
    }
}

impl From<&str> for VarName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<VarName> for String {
    fn from(val: VarName) -> Self {
        val.0
    }
}

impl From<&VarName> for String {
    fn from(val: &VarName) -> Self {
        val.0.clone()
    }
}

impl Display for VarName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

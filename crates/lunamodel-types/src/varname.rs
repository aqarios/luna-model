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

impl Into<String> for VarName {
    fn into(self) -> String {
        self.0
    }
}

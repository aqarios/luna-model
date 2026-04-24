//! Internal helpers shared by Python solution views and iterators.

use std::hash::{Hash, Hasher};

use lunamodel_error::LunaModelResult;
use pyo3::{FromPyObject, PyAny, PyErr, exceptions::PyTypeError};

use crate::args::PyVarArg;
use crate::utils::VarKey;

impl From<String> for VarKey {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for VarKey {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(s) = obj.extract::<String>() {
            Ok(VarKey::Str(s))
        } else if let Ok(v) = obj.extract::<PyVarArg>() {
            Ok(VarKey::Var(v))
        } else {
            Err(PyTypeError::new_err("keys have to be 'str' or 'Variable'"))
        }
    }
}

impl Hash for VarKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            VarKey::Str(s) => s.hash(state),
            VarKey::Var(v) => v.v.name().unwrap().hash(state),
        }
    }
}

impl PartialEq<Self> for VarKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VarKey::Str(s1), VarKey::Str(s2)) => s1 == s2,
            (VarKey::Var(v1), VarKey::Var(v2)) => v1.v == v2.v,
            _ => false,
        }
    }
}

impl Eq for VarKey {}

impl VarKey {
    pub fn name(&self) -> LunaModelResult<String> {
        match self {
            Self::Str(v) => Ok(v.clone()),
            Self::Var(v) => v.v.name(),
        }
    }
}

//! FFI conversions for comparator / constraint-type enum wrappers.

use lunamodel_error::LunaModelError;
use lunamodel_unwind::*;
use pyo3::{PyResult, Python, pymethods};

use crate::{ffi::CapsuleFFI, types::PyCtype};

impl<'py> CapsuleFFI<'py, String> for PyCtype {
    fn to_capsule(&self, _: pyo3::Python<'py>) -> pyo3::PyResult<String> {
        Ok(match self {
            Self::Unconstrained => "builtins.capsule.ctype.unconstrained".to_owned(),
            Self::Equality => "builtins.capsule.ctype.equality".to_owned(),
            Self::Inequality => "builtins.capsule.ctype.inequality".to_owned(),
            Self::LessEqual => "builtins.capsule.ctype.less_equal".to_owned(),
            Self::GreaterEqual => "builtins.capsule.ctype.greater_equal".to_owned(),
        })
    }

    fn from_capsule(capsule: String) -> pyo3::PyResult<Self> {
        match capsule.as_str() {
            "builtins.capsule.ctype.unconstrained" => Ok(Self::Unconstrained),
            "builtins.capsule.ctype.equality" => Ok(Self::Equality),
            "builtins.capsule.ctype.inequality" => Ok(Self::Inequality),
            "builtins.capsule.ctype.less_equal" => Ok(Self::LessEqual),
            "builtins.capsule.ctype.greater_equal" => Ok(Self::GreaterEqual),
            _ => Err(LunaModelError::Internal(
                format!("unknown ctype capsule: {capsule}").into(),
            ))?,
        }
    }
}

#[unwindable]
#[pymethods]
impl PyCtype {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<String> {
        self.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: String) -> PyResult<Self> {
        Self::from_capsule(capsule)
    }
}

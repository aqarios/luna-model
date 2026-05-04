//! FFI conversions for comparator enum wrappers.

use lunamodel_error::LunaModelError;
use lunamodel_unwind::*;
use pyo3::{PyResult, Python, pymethods};

use crate::{ffi::CapsuleFFI, types::PyComparator};

impl<'py> CapsuleFFI<'py, String> for PyComparator {
    fn to_capsule(&self, _: pyo3::Python<'py>) -> pyo3::PyResult<String> {
        Ok(match self {
            Self::Eq => "builtins.capsule.comparator.eq".to_owned(),
            Self::Le => "builtins.capsule.comparator.le".to_owned(),
            Self::Ge => "builtins.capsule.comparator.ge".to_owned(),
        })
    }

    fn from_capsule(capsule: String) -> pyo3::PyResult<Self> {
        match capsule.as_str() {
            "builtins.capsule.comparator.eq" => Ok(Self::Eq),
            "builtins.capsule.comparator.le" => Ok(Self::Le),
            "builtins.capsule.comparator.ge" => Ok(Self::Ge),
            _ => Err(LunaModelError::Internal(
                format!("unknown comparator capsule: {capsule}").into(),
            ))?,
        }
    }
}

#[unwindable]
#[pymethods]
impl PyComparator {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<String> {
        self.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: String) -> PyResult<Self> {
        Self::from_capsule(capsule)
    }
}

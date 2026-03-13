use lunamodel_error::LunaModelError;
use lunamodel_unwind::*;
use pyo3::{PyResult, Python, pymethods};

use crate::{ffi::CapsuleFFI, types::PySense};

impl<'py> CapsuleFFI<'py, String> for PySense {
    fn to_capsule(&self, _: pyo3::Python<'py>) -> pyo3::PyResult<String> {
        Ok(match self {
            Self::Min => "builtins.capsule.sense.min".to_owned(),
            Self::Max => "builtins.capsule.sense.max".to_owned(),
        })
    }

    fn from_capsule(capsule: String) -> pyo3::PyResult<Self> {
        match capsule.as_str() {
            "builtins.capsule.sense.min" => Ok(Self::Min),
            "builtins.capsule.sense.max" => Ok(Self::Max),
            _ => Err(LunaModelError::Internal(
                format!("unknown sense capsule: {capsule}").into(),
            ))?,
        }
    }
}

#[unwindable]
#[pymethods]
impl PySense {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<String> {
        self.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: String) -> PyResult<Self> {
        Self::from_capsule(capsule)
    }
}

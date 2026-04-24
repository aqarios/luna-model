//! FFI conversions for solution value-source enum wrappers.

use lunamodel_error::LunaModelError;
use lunamodel_unwind::*;
use pyo3::{PyResult, Python, pymethods};

use crate::{ffi::CapsuleFFI, types::PyValueSource};

impl<'py> CapsuleFFI<'py, String> for PyValueSource {
    fn to_capsule(&self, _: pyo3::Python<'py>) -> pyo3::PyResult<String> {
        Ok(match self {
            Self::Raw => "builtins.capsule.value_source.raw".to_owned(),
            Self::Obj => "builtins.capsule.value_source.obj".to_owned(),
        })
    }

    fn from_capsule(capsule: String) -> pyo3::PyResult<Self> {
        match capsule.as_str() {
            "builtins.capsule.value_source.raw" => Ok(Self::Raw),
            "builtins.capsule.value_source.obj" => Ok(Self::Obj),
            _ => Err(LunaModelError::Internal(
                format!("unknown value_source capsule: {capsule}").into(),
            ))?,
        }
    }
}

#[unwindable]
#[pymethods]
impl PyValueSource {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<String> {
        self.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: String) -> PyResult<Self> {
        Self::from_capsule(capsule)
    }
}

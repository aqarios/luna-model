//! FFI conversions for translation-target enum wrappers.

use lunamodel_error::LunaModelError;
use lunamodel_unwind::*;
use pyo3::{PyResult, Python, pymethods};

use crate::{ffi::CapsuleFFI, types::PyTranslationTarget};

impl<'py> CapsuleFFI<'py, String> for PyTranslationTarget {
    fn to_capsule(&self, _: pyo3::Python<'py>) -> pyo3::PyResult<String> {
        Ok(match self {
            Self::Qubo => "builtins.capsule.translation_target.qubo".to_owned(),
            Self::Lp => "builtins.capsule.translation_target.lp".to_owned(),
            Self::Mps => "builtins.capsule.translation_target.mps".to_owned(),
            Self::Bqm => "builtins.capsule.translation_target.bqm".to_owned(),
            Self::Cqm => "builtins.capsule.translation_target.cqm".to_owned(),
        })
    }

    fn from_capsule(capsule: String) -> pyo3::PyResult<Self> {
        match capsule.as_str() {
            "builtins.capsule.translation_target.qubo" => Ok(Self::Qubo),
            "builtins.capsule.translation_target.lp" => Ok(Self::Lp),
            "builtins.capsule.translation_target.mps" => Ok(Self::Mps),
            "builtins.capsule.translation_target.bqm" => Ok(Self::Bqm),
            "builtins.capsule.translation_target.cqm" => Ok(Self::Cqm),
            _ => Err(LunaModelError::Internal(
                format!("unknown translation_target capsule: {capsule}").into(),
            ))?,
        }
    }
}

#[unwindable]
#[pymethods]
impl PyTranslationTarget {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<String> {
        self.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: String) -> PyResult<Self> {
        Self::from_capsule(capsule)
    }
}

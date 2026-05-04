//! FFI conversions for variable-type enum wrappers.

use lunamodel_error::LunaModelError;
use lunamodel_unwind::*;
use pyo3::{PyResult, Python, pymethods};

use crate::{ffi::CapsuleFFI, types::PyVtype};

impl<'py> CapsuleFFI<'py, String> for PyVtype {
    fn to_capsule(&self, _: pyo3::Python<'py>) -> pyo3::PyResult<String> {
        Ok(match self {
            Self::Binary => "builtins.capsule.vtype.binary".to_owned(),
            Self::InvertedBinary => "builtins.capsule.vtype.inverted_binary".to_owned(),
            Self::Spin => "builtins.capsule.vtype.spin".to_owned(),
            Self::Real => "builtins.capsule.vtype.real".to_owned(),
            Self::Integer => "builtins.capsule.vtype.integer".to_owned(),
        })
    }

    fn from_capsule(capsule: String) -> pyo3::PyResult<Self> {
        match capsule.as_str() {
            "builtins.capsule.vtype.binary" => Ok(Self::Binary),
            "builtins.capsule.vtype.inverted_binary" => Ok(Self::InvertedBinary),
            "builtins.capsule.vtype.spin" => Ok(Self::Spin),
            "builtins.capsule.vtype.real" => Ok(Self::Real),
            "builtins.capsule.vtype.integer" => Ok(Self::Integer),
            _ => Err(LunaModelError::Internal(
                format!("unknown vtype capsule: {capsule}").into(),
            ))?,
        }
    }
}

#[unwindable]
#[pymethods]
impl PyVtype {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<String> {
        self.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: String) -> PyResult<Self> {
        Self::from_capsule(capsule)
    }
}

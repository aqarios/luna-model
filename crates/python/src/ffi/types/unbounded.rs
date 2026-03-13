use lunamodel_error::LunaModelError;
use lunamodel_unwind::*;
use pyo3::{PyResult, Python, pymethods};

use crate::{PyUnbounded, ffi::CapsuleFFI};

impl<'py> CapsuleFFI<'py, String> for PyUnbounded {
    fn to_capsule(&self, _: pyo3::Python<'py>) -> pyo3::PyResult<String> {
        Ok("builtins.capsule.unbounded".to_owned())
    }

    fn from_capsule(capsule: String) -> pyo3::PyResult<Self> {
        match capsule.as_str() {
            "builtins.capsule.unbounded" => Ok(Self {}),
            _ => Err(LunaModelError::Internal(
                format!("unknown unbounded capsule: {capsule}").into(),
            ))?,
        }
    }
}

#[unwindable]
#[pymethods]
impl PyUnbounded {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<String> {
        self.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: String) -> PyResult<Self> {
        Self::from_capsule(capsule)
    }
}
// PyUnbounded

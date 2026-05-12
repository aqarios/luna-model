//! Low-level capsule and pointer FFI helpers for Python environment wrappers.

use std::ffi::CStr;

use lunamodel_core::ArcEnv;
use lunamodel_unwind::*;
use pyo3::{prelude::*, types::PyCapsule};

use crate::{PyEnvironment, ffi::CapsuleFFI};

const CAPUSULE_NAME_ENV: &CStr = c"builtins.capsule.PyEnvironment";

impl<'py> CapsuleFFI<'py> for ArcEnv {
    fn to_capsule(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        PyCapsule::new(py, self.clone(), Some(CAPUSULE_NAME_ENV.to_owned()))
    }

    fn from_capsule(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        let ptr = capsule.pointer_checked(Some(CAPUSULE_NAME_ENV))?;
        let arc: ArcEnv = unsafe { ptr.cast::<ArcEnv>().as_ref().clone() };
        Ok(arc)
    }
}

#[unwindable]
#[pymethods]
impl PyEnvironment {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        self.env.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        Ok(PyEnvironment {
            env: ArcEnv::from_capsule(capsule)?,
        })
    }
}

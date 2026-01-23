use std::ffi::CStr;

use lunamodel_core::ArcEnv;
use lunamodel_unwind::*;
use pyo3::{prelude::*, types::PyCapsule};

use crate::PyEnvironment;

const CAPUSULE_NAME_ENV: &CStr = c"builtins.capsule.PyEnvironment";

#[unwindable]
#[pymethods]
impl PyEnvironment {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        PyCapsule::new(py, self.env.clone(), Some(CAPUSULE_NAME_ENV.to_owned()))
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: &Bound<'py, PyCapsule>) -> PyResult<Self> {
        let ptr = capsule.pointer_checked(Some(CAPUSULE_NAME_ENV))?;
        let arc: ArcEnv = unsafe { ptr.cast::<ArcEnv>().as_ref().clone() };
        Ok(PyEnvironment { env: arc })
    }
}

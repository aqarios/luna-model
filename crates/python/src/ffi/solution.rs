use parking_lot::RwLock;
use std::{ffi::CStr, sync::Arc};

use lunamodel_core::Solution;
use lunamodel_unwind::*;
use pyo3::{prelude::*, types::PyCapsule};

use crate::PySolution;

const CAPUSULE_NAME_ENV: &CStr = c"builtins.capsule.PySolution";

#[unwindable]
#[pymethods]
impl PySolution {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        PyCapsule::new(py, self.s.clone(), Some(CAPUSULE_NAME_ENV.to_owned()))
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: &Bound<'py, PyCapsule>) -> PyResult<Self> {
        let ptr = capsule.pointer_checked(Some(CAPUSULE_NAME_ENV))?;
        let arc: Arc<RwLock<Solution>> =
            unsafe { ptr.cast::<Arc<RwLock<Solution>>>().as_ref().clone() };
        Ok(PySolution { s: arc })
    }
}

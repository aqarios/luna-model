//! Low-level capsule and pointer FFI helpers for Python solution wrappers.

use parking_lot::RwLock;
use std::{ffi::CStr, sync::Arc};

use lunamodel_core::Solution;
use lunamodel_unwind::*;
use pyo3::{prelude::*, types::PyCapsule};

use crate::{
    PySolution,
    ffi::{CapsuleFFI, capsule_name},
};

const CAPSULE_NAME_ENV: &CStr = c"builtins.capsule.PySolution";

impl<'py> CapsuleFFI<'py> for Arc<RwLock<Solution>> {
    fn to_capsule(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        PyCapsule::new_with_value(py, self.clone(), capsule_name(CAPSULE_NAME_ENV))
    }

    fn from_capsule(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        let ptr = capsule.pointer_checked(Some(capsule_name(CAPSULE_NAME_ENV)))?;
        let arc: Arc<RwLock<Solution>> =
            unsafe { ptr.cast::<Arc<RwLock<Solution>>>().as_ref().clone() };
        Ok(arc)
    }
}

#[unwindable]
#[pymethods]
impl PySolution {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        self.s.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        Ok(PySolution {
            s: Arc::<RwLock<Solution>>::from_capsule(capsule)?,
        })
    }
}

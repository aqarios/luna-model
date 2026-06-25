//! Low-level capsule and pointer FFI helpers for Python pass context wrappers.

use std::{ffi::CStr, sync::Arc};

use lunamodel_transpiler::AnalysisManager;
use lunamodel_unwind::*;
use pyo3::{
    Bound, PyResult, Python, pymethods,
    types::{PyCapsule, PyCapsuleMethods},
};

use crate::{
    ffi::{CapsuleFFI, capsule_name},
    transform::PyPassContext,
};

const CAPSULE_NAME_PCTX: &CStr = c"builtins.capsule.PyPassContext";

impl<'py> CapsuleFFI<'py> for Arc<AnalysisManager> {
    fn to_capsule(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        PyCapsule::new_with_value(py, self.clone(), capsule_name(CAPSULE_NAME_PCTX))
    }

    fn from_capsule(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        let ptr = capsule.pointer_checked(Some(capsule_name(CAPSULE_NAME_PCTX)))?;
        let manager = unsafe { ptr.cast::<Arc<AnalysisManager>>().as_ref().clone() };
        Ok(manager)
    }
}

#[unwindable]
#[pymethods]
impl PyPassContext {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        self.manager.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        Ok(Self {
            manager: Arc::<AnalysisManager>::from_capsule(capsule)?,
        })
    }
}

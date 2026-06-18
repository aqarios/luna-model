//! Low-level capsule and pointer FFI helpers for Python bounds wrappers.

use std::ffi::CStr;
use std::sync::Arc;

use lunamodel_unwind::*;
use parking_lot::RwLock;
use pyo3::{
    Bound, PyResult, Python, pymethods,
    types::{PyCapsule, PyCapsuleMethods},
};

use crate::{PyBounds, PyBoundsContent, bounds::BoundsContent, ffi::CapsuleFFI};

const CAPUSULE_NAME_BOUNDS: &CStr = c"builtins.capsule.PyBounds";

impl<'py> CapsuleFFI<'py> for PyBoundsContent {
    fn to_capsule(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        PyCapsule::new_with_value(py, self.clone(), CAPUSULE_NAME_BOUNDS)
    }

    fn from_capsule(capsule: Bound<'py, PyCapsule>) -> PyResult<PyBoundsContent> {
        let ptr = capsule.pointer_checked(Some(CAPUSULE_NAME_BOUNDS))?;
        let content = unsafe { ptr.cast::<Arc<RwLock<BoundsContent>>>().as_ref().clone() };
        Ok(content)
    }
}

#[unwindable]
#[pymethods]
impl PyBounds {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        self.0.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        Ok(PyBounds(PyBoundsContent::from_capsule(capsule)?))
    }
}

//! Low-level capsule and pointer FFI helpers for Python specs wrappers.

use std::ffi::CStr;

use lunamodel_types::Specs;
use lunamodel_unwind::*;
use pyo3::{
    Bound, PyResult, Python, pymethods,
    types::{PyCapsule, PyCapsuleMethods},
};

use crate::ffi::{CapsuleFFI, capsule_name};
use crate::specs::PyModelSpecs;

const CAPSULE_NAME_SPECS: &CStr = c"builtins.capsule.PyModelSpecs";

impl<'py> CapsuleFFI<'py> for Specs {
    fn to_capsule(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        PyCapsule::new_with_value(py, self.clone(), capsule_name(CAPSULE_NAME_SPECS))
    }

    fn from_capsule(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        let ptr = capsule.pointer_checked(Some(capsule_name(CAPSULE_NAME_SPECS)))?;
        let s = unsafe { ptr.cast::<Specs>().as_ref().clone() };
        Ok(s)
    }
}

#[unwindable]
#[pymethods]
impl PyModelSpecs {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        self.s.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        Ok(Self {
            s: Specs::from_capsule(capsule)?,
        })
    }
}

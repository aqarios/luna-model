//! Low-level capsule and pointer FFI helpers for Python specs wrappers.

use std::ffi::CStr;

use lunamodel_types::Specs;
use lunamodel_unwind::*;
use pyo3::{
    Bound, PyResult, Python, pymethods,
    types::{PyCapsule, PyCapsuleMethods},
};

use crate::ffi::CapsuleFFI;
use crate::specs::PyModelSpecs;

const CAPUSULE_NAME_SPECS: &CStr = c"builtins.capsule.PyModelSpecs";

impl<'py> CapsuleFFI<'py> for Specs {
    fn to_capsule(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        PyCapsule::new(py, self.clone(), Some(CAPUSULE_NAME_SPECS.to_owned()))
    }

    fn from_capsule(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        let ptr = capsule.pointer_checked(Some(CAPUSULE_NAME_SPECS))?;
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

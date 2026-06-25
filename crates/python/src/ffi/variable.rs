//! Low-level capsule and pointer FFI helpers for Python variable wrappers.

use std::ffi::CStr;

use lunamodel_core::{ArcEnv, prelude::VarRef};
use lunamodel_unwind::*;
use pyo3::{
    Bound, PyResult, Python, pymethods,
    types::{PyCapsule, PyCapsuleMethods},
};

use crate::ffi::{CapsuleFFI, capsule_name};
use crate::variable::PyVariable;

const CAPSULE_NAME_VAR: &CStr = c"builtins.capsule.PyVar";

impl<'py> CapsuleFFI<'py, (u32, pyo3::Bound<'py, PyCapsule>)> for VarRef {
    fn to_capsule(&self, py: Python<'py>) -> PyResult<(u32, pyo3::Bound<'py, PyCapsule>)> {
        let capsule =
            PyCapsule::new_with_value(py, self.env.clone(), capsule_name(CAPSULE_NAME_VAR))?;
        Ok((self.id(), capsule))
    }

    fn from_capsule(capsule: (u32, pyo3::Bound<'py, PyCapsule>)) -> pyo3::PyResult<Self> {
        let (id, capsule) = capsule;
        let ptr = capsule.pointer_checked(Some(capsule_name(CAPSULE_NAME_VAR)))?;
        let env: ArcEnv = unsafe { ptr.cast::<ArcEnv>().as_ref().clone() };
        Ok(VarRef::new(id, env))
    }
}

#[unwindable]
#[pymethods]
impl PyVariable {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<(u32, Bound<'py, PyCapsule>)> {
        self.v.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: (u32, Bound<'py, PyCapsule>)) -> PyResult<Self> {
        Ok(Self {
            v: VarRef::from_capsule(capsule)?,
        })
    }
}

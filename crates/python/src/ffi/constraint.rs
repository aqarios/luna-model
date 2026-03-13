use std::{ffi::CStr, sync::Arc};

use lunamodel_core::prelude::Constraint;
use lunamodel_unwind::*;
use parking_lot::RwLock;
use pyo3::{
    Bound, PyResult, Python, pymethods,
    types::{PyCapsule, PyCapsuleMethods},
};

use crate::{PyConstraint, ffi::CapsuleFFI};

const CAPUSULE_NAME_C: &CStr = c"builtins.capsule.PyConstraint";

impl<'py> CapsuleFFI<'py> for PyConstraint {
    fn to_capsule(
        &self,
        py: pyo3::Python<'py>,
    ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyCapsule>> {
        PyCapsule::new(py, self.c.clone(), Some(CAPUSULE_NAME_C.to_owned()))
    }

    fn from_capsule(capsule: pyo3::Bound<'py, pyo3::types::PyCapsule>) -> pyo3::PyResult<Self> {
        let ptr = capsule.pointer_checked(Some(CAPUSULE_NAME_C))?;
        let constr = unsafe { ptr.cast::<Arc<RwLock<Constraint>>>().as_ref().clone() };
        Ok(Self { c: constr })
    }
}

#[unwindable]
#[pymethods]
impl PyConstraint {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        self.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        Ok(Self::from_capsule(capsule)?)
    }
}

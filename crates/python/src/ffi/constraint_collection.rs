//! Low-level capsule and pointer FFI helpers for Python constraint-collection wrappers.

use std::{ffi::CStr, sync::Arc};

use lunamodel_core::{ConstraintCollection, Model};
use lunamodel_unwind::*;
use parking_lot::RwLock;
use pyo3::{
    Bound, PyResult, Python,
    exceptions::PyValueError,
    pymethods,
    types::{PyCapsule, PyCapsuleMethods},
};

use crate::{
    PyConstraintCollection,
    ffi::{CapsuleFFI, capsule_name},
    prelude::PyConstraintCollectionContent as PyCCC,
};

const CAPSULE_NAME_CC: &CStr = c"builtins.capsule.PyConstraintCollectionContent.Cc";
const CAPSULE_NAME_MODEL: &CStr = c"builtins.capsule.PyConstraintCollectionContent.Model";

impl<'py> CapsuleFFI<'py> for PyCCC {
    fn to_capsule(
        &self,
        py: pyo3::Python<'py>,
    ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyCapsule>> {
        match &self {
            Self::Coll(arc_coll) => {
                PyCapsule::new_with_value(py, arc_coll.clone(), capsule_name(CAPSULE_NAME_CC))
            }

            Self::Model(arc_model) => {
                PyCapsule::new_with_value(py, arc_model.clone(), capsule_name(CAPSULE_NAME_MODEL))
            }
        }
    }

    fn from_capsule(capsule: pyo3::Bound<'py, pyo3::types::PyCapsule>) -> pyo3::PyResult<Self> {
        if let Ok(ptr) = capsule.pointer_checked(Some(capsule_name(CAPSULE_NAME_CC))) {
            let arc_cc = unsafe {
                ptr.cast::<Arc<RwLock<ConstraintCollection>>>()
                    .as_ref()
                    .clone()
            };
            Ok(Self::Coll(arc_cc))
        } else if let Ok(ptr) = capsule.pointer_checked(Some(capsule_name(CAPSULE_NAME_MODEL))) {
            let arc_model = unsafe { ptr.cast::<Arc<RwLock<Model>>>().as_ref().clone() };
            Ok(Self::Model(arc_model))
        } else {
            Err(PyValueError::new_err(
                "input is an unexpected capsule type.",
            ))
        }
    }
}

#[unwindable]
#[pymethods]
impl PyConstraintCollection {
    pub fn _to_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        self.inner().to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(capsule: Bound<'py, PyCapsule>) -> PyResult<Self> {
        Ok(PyCCC::from_capsule(capsule)?.into())
    }
}

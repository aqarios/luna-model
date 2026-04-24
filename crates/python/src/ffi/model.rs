//! Low-level capsule and pointer FFI helpers for Python model wrappers.

use std::{ffi::CStr, sync::Arc};

use lunamodel_core::Model;
use lunamodel_unwind::*;
use parking_lot::RwLock;
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pymethods,
    types::{PyCapsule, PyCapsuleMethods},
};
use std::collections::HashMap;

use crate::{PyModel, PyModelMetadata, ffi::CapsuleFFI, model::PyModelContent};

const CAPUSULE_NAME_MODEL: &CStr = c"builtins.capsule.PyModel";
const CAPUSULE_NAME_MODEL_METADATA: &CStr = c"builtins.capsule.PyModelMetadata";

impl<'py> CapsuleFFI<'py, (pyo3::Bound<'py, PyCapsule>, pyo3::Bound<'py, PyCapsule>)>
    for PyModelContent
{
    fn to_capsule(
        &self,
        py: pyo3::Python<'py>,
    ) -> pyo3::PyResult<(pyo3::Bound<'py, PyCapsule>, pyo3::Bound<'py, PyCapsule>)> {
        let capsule = PyCapsule::new(py, self.m.clone(), Some(CAPUSULE_NAME_MODEL.to_owned()))?;
        let capsule_metadata = PyCapsule::new(
            py,
            self._metadata.data.clone(),
            Some(CAPUSULE_NAME_MODEL_METADATA.to_owned()),
        )?;
        Ok((capsule_metadata, capsule))
    }

    fn from_capsule(
        capsule: (pyo3::Bound<'py, PyCapsule>, pyo3::Bound<'py, PyCapsule>),
    ) -> pyo3::PyResult<Self> {
        let (capsule_metadata, capsule) = capsule;
        let ptr = capsule.pointer_checked(Some(CAPUSULE_NAME_MODEL))?;
        let ptr_metadata = capsule_metadata.pointer_checked(Some(CAPUSULE_NAME_MODEL_METADATA))?;
        let model = unsafe { ptr.cast::<Arc<RwLock<Model>>>().as_ref().clone() };
        let metadata = unsafe {
            ptr_metadata
                .cast::<Arc<RwLock<HashMap<String, Py<PyAny>>>>>()
                .as_ref()
                .clone()
        };
        Ok(Self {
            m: model,
            _metadata: PyModelMetadata { data: metadata },
        })
    }
}

#[unwindable]
#[pymethods]
impl PyModel {
    pub fn _to_capsule<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyCapsule>, Bound<'py, PyCapsule>)> {
        self.0.to_capsule(py)
    }

    #[staticmethod]
    pub fn _from_capsule<'py>(
        capsule: (Bound<'py, PyCapsule>, Bound<'py, PyCapsule>),
    ) -> PyResult<Self> {
        Ok(Self(PyModelContent::from_capsule(capsule)?))
    }
}

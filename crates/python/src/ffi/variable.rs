//! C-handle capsule transport for Python variable wrappers.

use lunamodel_capi_handle::{drop_handle, handle_from_storage, handle_into_storage};
use lunamodel_cext_pyffi::{new_capsule, take_handle};
use lunamodel_capi::LmVref;
use lunamodel_unwind::*;
use pyo3::{Bound, PyResult, Python, pymethods, types::PyCapsule};

use crate::PyVariable;

#[unwindable]
#[pymethods]
impl PyVariable {
    pub fn _to_c_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        let handle = handle_from_storage::<LmVref>(self.v.clone());
        new_capsule(py, handle, drop_handle::<LmVref>)
    }

    #[staticmethod]
    pub fn _from_c_capsule(capsule: Bound<'_, PyCapsule>) -> PyResult<Self> {
        let handle = take_handle::<LmVref>(&capsule)?;
        let vref = unsafe { handle_into_storage::<LmVref>(handle) }
            .expect("a consumed non-null variable handle must contain storage");
        Ok(Self::new(vref))
    }
}

//! C-handle capsule transport for Python model-specs wrappers.

use lunamodel_cext_handle::{drop_handle, handle_from_storage, handle_into_storage};
use lunamodel_cext_pyffi::{new_capsule, take_handle};
use lunamodel_capi::LmSpecs;
use lunamodel_unwind::*;
use pyo3::{Bound, PyResult, Python, pymethods, types::PyCapsule};

use crate::specs::PyModelSpecs;

#[unwindable]
#[pymethods]
impl PyModelSpecs {
    pub fn _to_c_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        let handle = handle_from_storage::<LmSpecs>(self.s.clone());
        new_capsule(py, handle, drop_handle::<LmSpecs>)
    }

    #[staticmethod]
    pub fn _from_c_capsule(capsule: Bound<'_, PyCapsule>) -> PyResult<Self> {
        let handle = take_handle::<LmSpecs>(&capsule)?;
        let specs = unsafe { handle_into_storage::<LmSpecs>(handle) }
            .expect("a consumed non-null specs handle must contain storage");
        Ok(Self { s: specs })
    }
}

//! C-handle capsule transport for Python constraint wrappers.

use lunamodel_capi_handle::{drop_handle, handle_from_shared, handle_into_shared};
use lunamodel_cext_pyffi::{new_capsule, take_handle};
use lunamodel_capi::LmConstraint;
use lunamodel_unwind::*;
use pyo3::{Bound, PyResult, Python, pymethods, types::PyCapsule};

use crate::PyConstraint;

#[unwindable]
#[pymethods]
impl PyConstraint {
    pub fn _to_c_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        let handle = handle_from_shared::<LmConstraint>(self.c.clone());
        new_capsule(py, handle, drop_handle::<LmConstraint>)
    }

    #[staticmethod]
    pub fn _from_c_capsule(capsule: Bound<'_, PyCapsule>) -> PyResult<Self> {
        let handle = take_handle::<LmConstraint>(&capsule)?;
        let c = unsafe { handle_into_shared::<LmConstraint>(handle) }
            .expect("a consumed non-null constraint handle must contain shared storage");
        Ok(Self { c })
    }
}

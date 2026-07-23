//! C-handle capsule transport for Python solution wrappers.

use lunamodel_capi_handle::{drop_handle, handle_from_shared, handle_into_shared};
use lunamodel_capi_pyffi::{new_capsule, take_handle};
use lunamodel_capi::LmSol;
use lunamodel_unwind::*;
use pyo3::{Bound, PyResult, Python, pymethods, types::PyCapsule};

use crate::PySolution;

#[unwindable]
#[pymethods]
impl PySolution {
    pub fn _to_c_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        let handle = handle_from_shared::<LmSol>(self.s.clone());
        new_capsule(py, handle, drop_handle::<LmSol>)
    }

    #[staticmethod]
    pub fn _from_c_capsule(capsule: Bound<'_, PyCapsule>) -> PyResult<Self> {
        let handle = take_handle::<LmSol>(&capsule)?;
        let solution = unsafe { handle_into_shared::<LmSol>(handle) }
            .expect("a consumed non-null solution handle must contain shared storage");
        Ok(Self { s: solution })
    }
}

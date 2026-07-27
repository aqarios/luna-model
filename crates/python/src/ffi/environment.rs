//! C-handle capsule transport for Python environment wrappers.

use lunamodel_capi::LmEnv;
use lunamodel_capi_handle::{drop_handle, handle_from_storage, handle_into_storage};
use lunamodel_capi_pyffi::{new_capsule, take_handle};
use lunamodel_unwind::*;
use pyo3::{Bound, PyResult, Python, pymethods, types::PyCapsule};

use crate::PyEnvironment;

#[unwindable]
#[pymethods]
impl PyEnvironment {
    pub fn _to_c_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        let handle = handle_from_storage::<LmEnv>(self.env.clone());
        new_capsule(py, handle, drop_handle::<LmEnv>)
    }

    #[staticmethod]
    pub fn _from_c_capsule(capsule: Bound<'_, PyCapsule>) -> PyResult<Self> {
        let handle = take_handle::<LmEnv>(&capsule)?;
        let env = unsafe { handle_into_storage::<LmEnv>(handle) }
            .expect("a consumed non-null environment handle must contain shared storage");
        Ok(Self { env })
    }
}

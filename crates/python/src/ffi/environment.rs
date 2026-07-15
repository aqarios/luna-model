//! C-handle capsule transport for Python environment wrappers.

use lunamodel_cext_handle::{drop_handle, handle_into_shared, new_handle};
use lunamodel_cext_pyffi::{new_capsule, take_handle};
use lunamodel_cext_vtable::LmEnv;
use lunamodel_unwind::*;
use pyo3::{Bound, PyResult, Python, pymethods, types::PyCapsule};

use crate::PyEnvironment;

#[unwindable]
#[pymethods]
impl PyEnvironment {
    pub fn _to_c_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        // ArcEnv already owns synchronized core storage. The outer LmEnv
        // handle storage is temporary transport around a clone of that Arc.
        let handle = new_handle::<LmEnv>(self.env.clone());
        new_capsule(py, handle, drop_handle::<LmEnv>)
    }

    #[staticmethod]
    pub fn _from_c_capsule(capsule: Bound<'_, PyCapsule>) -> PyResult<Self> {
        let handle = take_handle::<LmEnv>(&capsule)?;
        let shared = unsafe { handle_into_shared::<LmEnv>(handle) }
            .expect("a consumed non-null environment handle must contain shared storage");
        let env = shared.read_arc().clone();
        Ok(Self { env })
    }
}

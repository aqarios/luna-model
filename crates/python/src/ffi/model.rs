//! C-handle capsule transport for Python model wrappers.

use lunamodel_cext_handle::{drop_handle, handle_from_shared, handle_into_shared};
use lunamodel_cext_pyffi::{new_capsule, take_handle};
use lunamodel_cext_vtable::LmModel;
use lunamodel_unwind::*;
use pyo3::{Bound, PyResult, Python, pymethods, types::PyCapsule};

use crate::{PyModel, PyModelMetadata, model::PyModelContent};

#[unwindable]
#[pymethods]
impl PyModel {
    pub fn _to_c_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        let handle = handle_from_shared::<LmModel>(self.m.clone());
        new_capsule(py, handle, drop_handle::<LmModel>)
    }

    #[staticmethod]
    pub fn _from_c_capsule(capsule: Bound<'_, PyCapsule>) -> PyResult<Self> {
        let handle = take_handle::<LmModel>(&capsule)?;
        // `take_handle` rejected null and transferred the owned host handle.
        let model = unsafe { handle_into_shared::<LmModel>(handle) }
            .expect("a consumed non-null model handle must contain shared storage");

        Ok(Self(PyModelContent {
            m: model,
            _metadata: PyModelMetadata::default(),
        }))
    }
}

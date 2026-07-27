//! C-handle capsule transport for Python constraint-collection wrappers.

use lunamodel_capi::LmCC;
use lunamodel_capi_handle::{
    ConstraintCollectionHandle, drop_handle, handle_from_storage, handle_into_storage,
};
use lunamodel_capi_pyffi::{new_capsule, take_handle};
use lunamodel_unwind::*;
use pyo3::{Bound, PyResult, Python, pymethods, types::PyCapsule};

use crate::{PyConstraintCollection, prelude::PyConstraintCollectionContent as PyCCC};

#[unwindable]
#[pymethods]
impl PyConstraintCollection {
    pub fn _to_c_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        let storage = match self.inner() {
            PyCCC::Coll(collection) => ConstraintCollectionHandle::Collection(collection.clone()),
            PyCCC::Model(model) => ConstraintCollectionHandle::Model(model.clone()),
        };
        let handle = handle_from_storage::<LmCC>(storage);
        new_capsule(py, handle, drop_handle::<LmCC>)
    }

    #[staticmethod]
    pub fn _from_c_capsule(capsule: Bound<'_, PyCapsule>) -> PyResult<Self> {
        let handle = take_handle::<LmCC>(&capsule)?;
        let storage = unsafe { handle_into_storage::<LmCC>(handle) }
            .expect("a consumed non-null constraint-collection handle must contain storage");
        let content = match storage {
            ConstraintCollectionHandle::Collection(collection) => PyCCC::Coll(collection),
            ConstraintCollectionHandle::Model(model) => PyCCC::Model(model),
        };
        Ok(content.into())
    }
}

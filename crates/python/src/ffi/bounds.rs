//! C-handle capsule transport for Python bounds wrappers.

use lunamodel_cext_handle::{drop_handle, handle_from_storage, handle_into_storage};
use lunamodel_cext_pyffi::{new_capsule, take_handle};
use lunamodel_capi::LmLazyBounds;
use lunamodel_core::prelude::LazyBounds;
use lunamodel_unwind::*;
use pyo3::{Bound, PyResult, Python, pymethods, types::PyCapsule};

use crate::{PyBounds, bounds::BoundsContent};

#[unwindable]
#[pymethods]
impl PyBounds {
    pub fn _to_c_capsule<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyCapsule>> {
        let bounds: LazyBounds = self.0.read().clone().into();
        let handle = handle_from_storage::<LmLazyBounds>(bounds);
        new_capsule(py, handle, drop_handle::<LmLazyBounds>)
    }

    #[staticmethod]
    pub fn _from_c_capsule(capsule: Bound<'_, PyCapsule>) -> PyResult<Self> {
        let handle = take_handle::<LmLazyBounds>(&capsule)?;
        let bounds = unsafe { handle_into_storage::<LmLazyBounds>(handle) }
            .expect("a consumed non-null bounds handle must contain storage");
        Ok(BoundsContent::Lazy(bounds).into())
    }
}

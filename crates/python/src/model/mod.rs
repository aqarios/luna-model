mod access;
mod construction;
mod general;
mod io;
mod metadata;
mod modification;
mod ser;
mod setter;

use std::sync::Arc;

use lunamodel_core::prelude::Model;
use parking_lot::RwLock;
use pyo3::pyclass;

pub use metadata::PyModelMetadata;

// #[pyclass(subclass, name = "Model", module = "luna_model._core")]
#[pyclass]
#[repr(C)]
pub struct PyModel {
    pub m: Arc<RwLock<Model>>,
    #[pyo3(get, set)]
    pub _metadata: PyModelMetadata,
}

impl Clone for PyModel {
    fn clone(&self) -> Self {
        PyModel {
            m: Arc::new(RwLock::new(self.m.read().clone())),
            _metadata: PyModelMetadata::new(),
        }
    }
}

impl From<Model> for PyModel {
    fn from(value: Model) -> Self {
        Self {
            m: Arc::new(RwLock::new(value)),
            _metadata: PyModelMetadata::new(),
        }
    }
}

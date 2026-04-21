mod access;
mod construction;
mod general;
mod io;
mod metadata;
mod modification;
mod ser;
mod setter;

use std::sync::Arc;

use derive_more::{Deref, DerefMut};
use lunamodel_core::prelude::Model;
use parking_lot::RwLock;
use pyo3::pyclass;

pub use metadata::PyModelMetadata;

#[derive(Debug)]
pub struct PyModelContent {
    pub m: Arc<RwLock<Model>>,
    pub _metadata: PyModelMetadata,
}

// #[pyclass(subclass, name = "Model", module = "luna_model._core")]
#[pyclass]
#[repr(C)]
#[derive(Deref, DerefMut, Debug)]
pub struct PyModel(pub PyModelContent);

impl Clone for PyModel {
    fn clone(&self) -> Self {
        PyModel(PyModelContent {
            m: Arc::new(RwLock::new(self.m.read().clone())),
            _metadata: PyModelMetadata::new(),
        })
    }
}

impl From<Model> for PyModel {
    fn from(value: Model) -> Self {
        Self(PyModelContent {
            m: Arc::new(RwLock::new(value)),
            _metadata: PyModelMetadata::new(),
        })
    }
}

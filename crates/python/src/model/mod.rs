//! Python wrapper for models and model metadata.
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

/// Shared content behind the Python model wrapper.
#[derive(Debug)]
pub struct PyModelContent {
    /// Shared core model handle.
    pub m: Arc<RwLock<Model>>,
    /// Python-side metadata not stored in the core model.
    pub _metadata: PyModelMetadata,
}

#[pyclass(from_py_object)]
#[repr(C)]
#[derive(Deref, DerefMut, Debug)]
pub struct PyModel(pub PyModelContent);

impl PyModel {
    pub fn inner(&self) -> Arc<RwLock<Model>> {
        Arc::clone(&self.m)
    }
}

impl Clone for PyModel {
    /// Clones the model by deep-cloning the underlying core model.
    fn clone(&self) -> Self {
        PyModel(PyModelContent {
            m: Arc::new(RwLock::new(self.m.read().clone())),
            _metadata: PyModelMetadata::new(),
        })
    }
}

impl From<Model> for PyModel {
    /// Wraps an owned core model for Python.
    fn from(value: Model) -> Self {
        Self(PyModelContent {
            m: Arc::new(RwLock::new(value)),
            _metadata: PyModelMetadata::new(),
        })
    }
}

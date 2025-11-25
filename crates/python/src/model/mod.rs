mod ser;
mod construction;

use lunamodel_core::prelude::Model;
use parking_lot::RwLock;
use pyo3::pyclass;

// #[pyclass(subclass, name = "Model", module = "luna_model._core")]
#[pyclass]
#[repr(transparent)]
pub struct PyModel {
    pub m: RwLock<Model>,
}

impl Clone for PyModel {
    fn clone(&self) -> Self {
        PyModel {
            m: RwLock::new(self.m.read().clone()),
        }
    }
}

impl From<Model> for PyModel {
    fn from(value: Model) -> Self {
        Self {
            m: RwLock::new(value),
        }
    }
}

impl From<PyModel> for Model {
    fn from(value: PyModel) -> Self {
        value.m.into_inner()
    }
}

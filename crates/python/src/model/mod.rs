mod access;
mod construction;
mod ser;

use std::sync::Arc;

use lunamodel_core::prelude::Model;
use parking_lot::RwLock;
use pyo3::pyclass;

// #[pyclass(subclass, name = "Model", module = "luna_model._core")]
#[pyclass]
#[repr(C)]
pub struct PyModel {
    pub m: Arc<RwLock<Model>>,
}

impl Clone for PyModel {
    fn clone(&self) -> Self {
        PyModel {
            m: Arc::new(RwLock::new(self.m.read().clone())),
        }
    }
}

impl From<Model> for PyModel {
    fn from(value: Model) -> Self {
        Self {
            m: Arc::new(RwLock::new(value)),
        }
    }
}

// impl From<PyModel> for Model {
//     fn from(value: PyModel) -> Self {
//         value.m.into_inner()
//     }
// }

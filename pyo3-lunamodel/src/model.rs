use std::sync::Arc;

use luna_model::{
    core::Model,
    python::{PyModelMetadata, prelude::PyModelContent},
};
use parking_lot::RwLock;

#[repr(transparent)]
pub struct PyModel(pub PyModelContent);

impl PyModel {
    pub fn inner(&self) -> Arc<RwLock<Model>> {
        Arc::clone(&self.0.m)
    }
}

impl From<Model> for PyModel {
    fn from(value: Model) -> Self {
        Self(PyModelContent {
            m: Arc::new(RwLock::new(value)),
            _metadata: PyModelMetadata::default(),
        })
    }
}

capsule_wrapper! {
    wrapper: PyModel,
    public: Model,
    inner: PyModelContent,
    attr: "_m",
    from_py: "_from_pym",
}

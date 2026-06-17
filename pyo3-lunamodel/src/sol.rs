use std::sync::Arc;

use derive_more::{Deref, DerefMut};
use lunamodel::core::Solution;
use parking_lot::RwLock;

#[derive(Deref, DerefMut)]
#[repr(transparent)]
/// A wrapper around a [`Arc<RwLock<Solution>>`] that can be converted to and from python with `pyo3`.
// pub struct PySolution(pub Arc<RwLock<Solution>>);
pub struct PySolution(pub Arc<RwLock<Solution>>);

impl PySolution {
    pub fn inner(&self) -> Arc<RwLock<Solution>> {
        Arc::clone(&self.0)
    }
}

impl From<Solution> for PySolution {
    fn from(value: Solution) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }
}

capsule_wrapper! {
    wrapper: PySolution,
    public: Solution,
    inner: Arc<RwLock<Solution>>,
    attr: "_s",
    from_py: "_from_pys",
}

use std::sync::Arc;

use lunamodel_core::Solution;
use parking_lot::RwLock;

#[repr(transparent)]
/// A wrapper around a [`Arc<RwLock<Solution>>`] that can be converted to and from python with `pyo3`.
pub struct PySolution(pub Arc<RwLock<Solution>>);

capsule_wrapper! {
    wrapper: PySolution,
    public: Solution,
    inner: Arc<RwLock<Solution>>,
    attr: "_s",
    from_py: "_from_pys",
}

use std::sync::Arc;

use lunamodel::core::prelude::Constraint;
use parking_lot::RwLock;

#[repr(transparent)]
/// A wrapper around a [`Constraint`] that can be converted to and from python with `pyo3`.
pub struct PyConstraint(pub Arc<RwLock<Constraint>>);

capsule_wrapper! {
    wrapper: PyConstraint,
    public: Constraint,
    inner: Arc<RwLock<Constraint>>,
    attr: "_c",
    from_py: "_from_pyc",
}

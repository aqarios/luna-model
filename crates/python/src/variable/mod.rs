use lunamodel_core::prelude::VarRef;
use parking_lot::RwLock;
use pyo3::pyclass;

// #[pyclass(subclass, name = "Model", module = "luna_model._core")]
#[pyclass]
#[repr(transparent)]
pub struct PyVariable {
    pub v: RwLock<VarRef>,
}

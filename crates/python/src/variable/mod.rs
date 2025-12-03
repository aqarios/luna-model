mod access;
mod cmp;
mod creation;
mod io;
mod ops;

use lunamodel_core::prelude::VarRef;
use pyo3::pyclass;

// #[pyclass(subclass, name = "Model", module = "luna_model._core")]
#[pyclass]
#[repr(C)]
#[derive(Clone)]
pub struct PyVariable {
    pub v: VarRef,
}

impl PyVariable {
    fn new(vref: VarRef) -> Self {
        Self { v: vref }
    }
}

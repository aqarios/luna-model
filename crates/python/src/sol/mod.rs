use std::sync::Arc;

use lunamodel_core::Solution;
use parking_lot::RwLock;
use pyo3::pyclass;

#[pyclass]
#[repr(C)]
pub struct PySolution {
    pub s: Arc<RwLock<Solution>>,
}

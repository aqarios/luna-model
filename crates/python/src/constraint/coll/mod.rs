mod access;
mod creation;

use std::sync::Arc;

use lunamodel_core::ConstraintCollection;
use parking_lot::RwLock;
use pyo3::pyclass;

#[pyclass]
pub struct PyConstraintCollection {
    c: Arc<RwLock<ConstraintCollection>>,
}

impl PyConstraintCollection {
    pub fn new(coll: ConstraintCollection) -> Self {
        Self {
            c: Arc::new(RwLock::new(coll)),
        }
    }
}

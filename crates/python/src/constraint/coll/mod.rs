mod access;
mod creation;
mod io;
mod iter;
mod modification;
mod ser;

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

impl Into<PyConstraintCollection> for ConstraintCollection {
    fn into(self) -> PyConstraintCollection {
        PyConstraintCollection::new(self)
    }
}

impl Into<ConstraintCollection> for &PyConstraintCollection {
    fn into(self) -> ConstraintCollection {
        self.c.read_arc().clone()
    }
}

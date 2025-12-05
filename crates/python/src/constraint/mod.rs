mod access;
mod cmp;
mod creation;

use std::sync::Arc;

use lunamodel_core::{ConstraintCollection, prelude::Constraint};
use parking_lot::RwLock;
use pyo3::pyclass;

#[pyclass]
pub struct PyConstraint {
    c: Arc<RwLock<Constraint>>,
}

impl PyConstraint {
    pub fn new(constr: Constraint) -> Self {
        Self {
            c: Arc::new(RwLock::new(constr)),
        }
    }
}

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

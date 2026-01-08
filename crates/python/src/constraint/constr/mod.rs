mod access;
mod cmp;
mod creation;
mod io;

use std::sync::Arc;

use lunamodel_core::prelude::Constraint;
use parking_lot::RwLock;
use pyo3::pyclass;

#[pyclass]
#[derive(Clone)]
pub struct PyConstraint {
    pub c: Arc<RwLock<Constraint>>,
}

impl PyConstraint {
    pub fn new(constr: Constraint) -> Self {
        Self {
            c: Arc::new(RwLock::new(constr)),
        }
    }
}

impl From<&Constraint> for PyConstraint {
    fn from(constr: &Constraint) -> Self {
        constr.clone().into()
    }
}

impl From<Constraint> for PyConstraint {
    fn from(constr: Constraint) -> Self {
        Self::new(constr)
    }
}

//! Python wrapper for individual constraints.
mod access;
mod cmp;
mod creation;
mod io;
mod setter;

use std::sync::Arc;

use lunamodel_core::prelude::Constraint;
use parking_lot::RwLock;
use pyo3::pyclass;

#[pyclass(subclass, from_py_object)]
#[derive(Clone, Debug)]
pub struct PyConstraint {
    /// Shared constraint storage.
    pub c: Arc<RwLock<Constraint>>,
}

impl PyConstraint {
    /// Creates a new Python constraint wrapper from an owned constraint.
    pub fn new(constr: Constraint) -> Self {
        Self {
            c: Arc::new(RwLock::new(constr)),
        }
    }
}

impl From<&Constraint> for PyConstraint {
    /// Clones a borrowed constraint into a Python wrapper.
    fn from(constr: &Constraint) -> Self {
        constr.clone().into()
    }
}

impl From<Constraint> for PyConstraint {
    /// Wraps an owned constraint as `PyConstraint`.
    fn from(constr: Constraint) -> Self {
        Self::new(constr)
    }
}

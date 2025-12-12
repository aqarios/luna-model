use crate::PyConstraint;

use super::PyConstraintCollection;

impl PyConstraintCollection {
    // todo: actually this should also return a view like object.
    pub fn constraint(&self, key: &str) -> PyConstraint {
        let constr = &(&self.c.read_arc())[key];
        PyConstraint::new(constr.clone())
    }
}

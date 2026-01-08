use pyo3::pyclass;

use crate::constraint::{PyConstraint, PyConstraintCollection};

#[pyclass]
pub struct PyConstraintCollectionIterator {
    items: Vec<(String, PyConstraint)>,
    idx: usize,
}

impl PyConstraintCollectionIterator {
    pub fn new(coll: &PyConstraintCollection) -> Self {
        Self {
            items: coll
                .c
                .read_arc()
                .iter()
                .map(|(name, constr)| (name.clone(), constr.into()))
                .collect(),
            idx: 0,
        }
    }
}

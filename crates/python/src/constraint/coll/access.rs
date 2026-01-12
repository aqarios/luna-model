use lunamodel_types::Comparator;
use pyo3::{PyResult, pymethods};

use crate::{PyConstraint, constraint::coll::iter::PyConstraintCollectionIterator};

use super::PyConstraintCollection;

#[pymethods]
impl PyConstraintCollection {
    // // todo: actually this should also return a view like object.
    // pub fn constraint(&self, key: &str) -> PyConstraint {
    //     let constr = &(&self.c.read_arc())[key];
    //     PyConstraint::new(constr.clone())
    // }

    fn items(&self) -> PyConstraintCollectionIterator {
        PyConstraintCollectionIterator::new(&self)
    }

    fn get(&self, key: String) -> PyResult<PyConstraint> {
        Ok(self.c.get(&key)?.clone().into())
    }

    fn ctypes(&self) -> Vec<Comparator> {
        self.c.ctypes()
    }

    fn equal_contents(&self, other: &Self) -> bool {
        self.c.equal_contents(&other.c)
    }

    fn __getitem__(&self, key: String) -> PyResult<PyConstraint> {
        self.get(key)
    }

    fn __len__(&self) -> usize {
        self.c.len()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.c.eq(&other.c)
    }

    fn __iter__(&self) -> PyConstraintCollectionIterator {
        self.items()
    }
}

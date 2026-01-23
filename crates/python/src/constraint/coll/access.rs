use lunamodel_types::Comparator;
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyConstraintCollection;
use crate::{PyConstraint, constraint::coll::iter::PyConstraintCollectionIterator};

#[unwindable]
#[pymethods]
impl PyConstraintCollection {
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
        PyConstraintCollectionIterator::new(&self)
    }
}

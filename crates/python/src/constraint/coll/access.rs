//! Accessors for Python constraint collections.

use lunamodel_core::prelude::ContentEquality;
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyConstraintCollection;
use crate::{
    PyConstraint, args::PyColArg, constraint::coll::iter::PyConstraintCollectionIterator,
    types::PyComparator,
};

#[unwindable]
#[pymethods]
impl PyConstraintCollection {
    fn items(&self) -> PyConstraintCollectionIterator {
        PyConstraintCollectionIterator::new(self)
    }

    fn get(&self, key: String) -> PyResult<PyConstraint> {
        Ok(self.read().get(&key)?.clone().into())
    }

    fn ctypes(&self) -> Vec<PyComparator> {
        self.read().ctypes().map(|c| c.into()).collect()
    }

    fn equal_contents(&self, other: PyColArg) -> bool {
        self.read().equal_contents(&other.read())
    }

    fn __getitem__(&self, key: String) -> PyResult<PyConstraint> {
        self.get(key)
    }

    fn __len__(&self) -> usize {
        self.read().len()
    }

    fn __eq__(&self, other: PyColArg) -> bool {
        self.read().eq(&other.read())
    }

    fn __iter__(&self) -> PyConstraintCollectionIterator {
        PyConstraintCollectionIterator::new(self)
    }

    fn __contains__(&self, cname: String) -> bool {
        self.read().contains(&cname)
    }
}

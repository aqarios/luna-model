//! Python wrapper for `ConstraintCollection`.
mod access;
mod content;
mod creation;
mod io;
mod iter;
mod modification;
mod ser;

pub mod utils;

use std::sync::Arc;

use lunamodel_core::{ConstraintCollection, Model};
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock};
use pyo3::pyclass;

pub use content::PyConstraintCollectionContent;
pub use iter::PyConstraintCollectionIterator;

/// Python-visible wrapper around either a standalone collection or a model-owned collection.
#[derive(Clone, Debug)]
#[pyclass(from_py_object)]
pub struct PyConstraintCollection {
    c: PyConstraintCollectionContent,
}

impl PyConstraintCollection {
    /// Returns the internal content handle.
    pub fn inner(&self) -> &PyConstraintCollectionContent {
        &self.c
    }

    /// Creates a collection wrapper bound to the constraints of a model.
    pub fn for_model(model: Arc<RwLock<Model>>) -> Self {
        Self {
            c: PyConstraintCollectionContent::Model(model),
        }
    }

    /// Creates a collection wrapper from an owned constraint collection.
    pub fn new(coll: ConstraintCollection) -> Self {
        Self {
            c: PyConstraintCollectionContent::Coll(Arc::new(RwLock::new(coll))),
        }
    }
}

impl From<ConstraintCollection> for PyConstraintCollection {
    /// Wraps an owned collection as `PyConstraintCollection`.
    fn from(val: ConstraintCollection) -> Self {
        PyConstraintCollection::new(val)
    }
}

impl From<&PyConstraintCollection> for ConstraintCollection {
    /// Clones the underlying collection out of the Python wrapper.
    fn from(val: &PyConstraintCollection) -> Self {
        val.read_with(|c| c.clone())
    }
}

impl From<PyConstraintCollectionContent> for PyConstraintCollection {
    /// Re-wraps an existing content handle.
    fn from(val: PyConstraintCollectionContent) -> Self {
        PyConstraintCollection { c: val }
    }
}

impl PyConstraintCollection {
    /// Borrows the underlying collection for read-only access.
    pub fn read(&self) -> MappedRwLockReadGuard<'_, ConstraintCollection> {
        self.c.read()
    }

    /// Borrows the underlying collection for mutable access.
    pub fn write(&mut self) -> MappedRwLockWriteGuard<'_, ConstraintCollection> {
        self.c.write()
    }

    /// Runs a closure against an immutable view of the collection.
    pub fn read_with<R>(&self, f: impl FnOnce(&ConstraintCollection) -> R) -> R {
        self.c.read_with(f)
    }

    /// Runs a closure against a mutable view of the collection.
    pub fn write_with<R>(&mut self, f: impl FnOnce(&mut ConstraintCollection) -> R) -> R {
        self.c.write_with(f)
    }
}

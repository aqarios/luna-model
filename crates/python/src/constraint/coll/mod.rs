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

#[derive(Clone, Debug)]
#[pyclass]
pub struct PyConstraintCollection {
    c: PyConstraintCollectionContent,
}

impl PyConstraintCollection {
    pub fn inner(&self) -> &PyConstraintCollectionContent {
        &self.c
    }

    pub fn for_model(model: Arc<RwLock<Model>>) -> Self {
        Self {
            c: PyConstraintCollectionContent::Model(model),
        }
    }

    pub fn new(coll: ConstraintCollection) -> Self {
        Self {
            c: PyConstraintCollectionContent::Coll(Arc::new(RwLock::new(coll))),
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
        self.read_with(|c| c.clone().into())
    }
}

impl Into<PyConstraintCollection> for PyConstraintCollectionContent {
    fn into(self) -> PyConstraintCollection {
        PyConstraintCollection { c: self }
    }
}

impl PyConstraintCollection {
    pub fn read(&self) -> MappedRwLockReadGuard<'_, ConstraintCollection> {
        self.c.read()
    }

    pub fn write(&mut self) -> MappedRwLockWriteGuard<'_, ConstraintCollection> {
        self.c.write()
    }

    pub fn read_with<R>(&self, f: impl FnOnce(&ConstraintCollection) -> R) -> R {
        self.c.read_with(f)
    }

    pub fn write_with<R>(&mut self, f: impl FnOnce(&mut ConstraintCollection) -> R) -> R {
        self.c.write_with(f)
    }
}

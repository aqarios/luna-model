mod access;
mod content;
mod creation;
mod io;
mod iter;
mod modification;
mod ser;

use std::sync::Arc;

use lunamodel_core::{ConstraintCollection, Model};
use parking_lot::RwLock;
use pyo3::pyclass;

pub use content::PyConstraintCollectionContent;
pub use iter::PyConstraintCollectionIterator;

#[pyclass]
pub struct PyConstraintCollection {
    pub c: PyConstraintCollectionContent,
}

impl PyConstraintCollection {
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
        (&self.c).into()
    }
}

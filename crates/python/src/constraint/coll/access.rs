use lunamodel_core::prelude::ContentEquality;
use lunamodel_types::Comparator;
use pyo3::pymethods;

use crate::{PyConstraint, constraint::coll::iter::PyConstraintCollectionIterator};

use super::PyConstraintCollection;

#[pymethods]
impl PyConstraintCollection {
    // // todo: actually this should also return a view like object.
    // pub fn constraint(&self, key: &str) -> PyConstraint {
    //     let constr = &(&self.c.read_arc())[key];
    //     PyConstraint::new(constr.clone())
    // }
    //
    fn items(&self) -> PyConstraintCollectionIterator {
        PyConstraintCollectionIterator::new(&self)
    }

    fn get(&self, key: String) -> PyConstraint {
        self.c.read_arc()[&key].clone().into()
    }

    fn ctypes(&self) -> Vec<Comparator> {
        self.c.read_arc().ctypes().collect()
    }

    fn equal_contents(&self, other: &Self) -> bool {
        self.c.read_arc().is_equal_contents(&other.c.read_arc())
    }

    fn __getitem__(&self, key: String) -> PyConstraint {
        self.get(key)
    }

    fn __len__(&self) -> usize {
        self.c.read_arc().len()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.c.read_arc().eq(&other.c.read_arc())
    }
}

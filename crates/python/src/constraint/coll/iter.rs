//! Iteration support for Python constraint collections.

use lunamodel_unwind::*;
use pyo3::{PyRef, PyRefMut, PyResult, pyclass, pymethods};

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
                .read()
                .iter()
                .map(|(name, constr)| (name.clone(), constr.into()))
                .collect(),
            idx: 0,
        }
    }
}

#[unwindable]
#[pymethods]
impl PyConstraintCollectionIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<(String, PyConstraint)>> {
        let res = slf.items.get(slf.idx);
        let out = match res {
            Option::None => Ok(None),
            Option::Some(val) => Ok(Some(val.clone())),
        };
        slf.idx += 1;
        out
    }
}

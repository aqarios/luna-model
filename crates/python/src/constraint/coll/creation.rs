use lunamodel_core::ConstraintCollection;
use lunamodel_unwind::unwindable;
use pyo3::pymethods;

use super::PyConstraintCollection;
use crate::unwind::unwind;

#[unwindable]
#[pymethods]
impl PyConstraintCollection {
    #[new]
    fn pynew() -> Self {
        Self::new(ConstraintCollection::default())
    }
}

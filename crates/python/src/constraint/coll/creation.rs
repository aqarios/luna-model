use lunamodel_core::ConstraintCollection;
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyConstraintCollection;

#[unwindable]
#[pymethods]
impl PyConstraintCollection {
    #[new]
    fn pynew() -> Self {
        Self::new(ConstraintCollection::default())
    }
}

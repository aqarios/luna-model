use pyo3::pymethods;

use super::PyConstraintCollection;

use lunamodel_core::ConstraintCollection;

#[pymethods]
impl PyConstraintCollection {
    #[new]
    fn pynew() -> Self {
        Self::new(ConstraintCollection::default())
    }
}

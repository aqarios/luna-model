use pyo3::pymethods;

use crate::bounds::{BoundsContent, unbounded::BoundValue};

use super::PyBounds;

#[pymethods]
impl PyBounds {
    #[getter]
    fn lower(&self) -> BoundValue {
        match &self.0 {
            BoundsContent::Concrete(conc) => conc.lower().into(),
            BoundsContent::Lazy(conc) => conc.lower().into(),
        }
    }

    #[getter]
    fn upper(&self) -> BoundValue {
        match &self.0 {
            BoundsContent::Concrete(conc) => conc.upper().into(),
            BoundsContent::Lazy(conc) => conc.upper().into(),
        }
    }
}

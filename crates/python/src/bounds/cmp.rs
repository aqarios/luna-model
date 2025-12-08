use lunamodel_types::Bound;
use pyo3::pymethods;

use crate::bounds::BoundsContent;

use super::PyBounds;

#[pymethods]
impl PyBounds {
    fn __eq__(&self, other: &Self) -> bool {
        let (self_lower, self_upper) = self.bs();
        let (other_lower, other_upper) = other.bs();
        self_lower == other_lower && self_upper == other_upper
    }
}

impl PyBounds {
    fn bs(self: &Self) -> (Option<Bound>, Option<Bound>) {
        match &self.0 {
            BoundsContent::Lazy(lazy) => (lazy.lower(), lazy.upper()),
            BoundsContent::Concrete(conc) => (Some(conc.lower()), Some(conc.upper())),
        }
    }
}

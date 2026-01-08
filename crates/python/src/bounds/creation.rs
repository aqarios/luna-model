use lunamodel_core::prelude::LazyBounds;
use pyo3::pymethods;

use crate::bounds::unbounded::BoundValue;

use super::PyBounds;

#[pymethods]
impl PyBounds {
    #[new]
    #[pyo3(signature=(lower=BoundValue::None, upper=BoundValue::None))]
    pub fn pynew(lower: BoundValue, upper: BoundValue) -> Self {
        PyBounds(super::BoundsContent::Lazy(LazyBounds::new(
            lower.into(),
            upper.into(),
        )))
    }
}

use lunamodel_core::prelude::LazyBounds;
use lunamodel_unwind::unwindable;
use pyo3::pymethods;

use super::PyBounds;
use crate::bounds::unbounded::BoundValue;
use crate::unwind::unwind;

#[unwindable]
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

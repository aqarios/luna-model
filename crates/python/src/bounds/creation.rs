//! Constructors for Python bounds wrappers.

use lunamodel_core::prelude::LazyBounds;
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyBounds;
use crate::bounds::unbounded::BoundValue;

#[unwindable]
#[pymethods]
impl PyBounds {
    #[new]
    #[pyo3(signature=(lower=BoundValue::None, upper=BoundValue::None))]
    pub fn pynew(lower: BoundValue, upper: BoundValue) -> Self {
        LazyBounds::new(lower.into(), upper.into()).into()
    }
}

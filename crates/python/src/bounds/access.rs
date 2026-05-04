//! Accessors for Python bounds wrappers.

use lunamodel_core::prelude::Bounds;
use lunamodel_unwind::*;
use pyo3::pymethods;

use crate::bounds::{BoundsContent, unbounded::BoundValue};

use super::PyBounds;

#[unwindable]
#[pymethods]
impl PyBounds {
    #[getter]
    fn lower(&self) -> BoundValue {
        match *self.0.read_arc() {
            BoundsContent::Concrete(conc) => conc.lower().into(),
            BoundsContent::Lazy(conc) => conc.lower().into(),
        }
    }

    #[getter]
    fn upper(&self) -> BoundValue {
        match *self.0.read_arc() {
            BoundsContent::Concrete(conc) => conc.upper().into(),
            BoundsContent::Lazy(conc) => conc.upper().into(),
        }
    }

    #[staticmethod]
    fn binary() -> Self {
        Bounds::binary().into()
    }

    #[staticmethod]
    fn spin() -> Self {
        Bounds::spin().into()
    }

    #[staticmethod]
    fn integer() -> Self {
        Bounds::integer().into()
    }

    #[staticmethod]
    fn real() -> Self {
        Bounds::real().into()
    }
}

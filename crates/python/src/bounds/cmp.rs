//! Equality and semantic-comparison helpers for Python bounds.

use lunamodel_types::Bound;
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyBounds;
use crate::{args::PyBoundsArg, bounds::BoundsContent};

#[unwindable]
#[pymethods]
impl PyBounds {
    fn __eq__(&self, other: PyBoundsArg) -> bool {
        let (self_lower, self_upper) = self.bs();
        let (other_lower, other_upper) = other.bs();
        self_lower == other_lower && self_upper == other_upper
    }
}

impl PyBounds {
    fn bs(&self) -> (Option<Bound>, Option<Bound>) {
        match *self.0.read_arc() {
            BoundsContent::Lazy(lazy) => (lazy.lower(), lazy.upper()),
            BoundsContent::Concrete(conc) => (Some(conc.lower()), Some(conc.upper())),
        }
    }
}

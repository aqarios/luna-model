//! Equality and semantic-comparison helpers for Python constraints.

use lunamodel_core::prelude::{Constraint, ContentEquality};
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyConstraint;
use crate::args::PyCArg;

#[unwindable]
#[pymethods]
impl PyConstraint {
    fn __eq__(&self, other: PyCArg) -> bool {
        let lhs: &Constraint = &self.c.read_arc();
        let rhs: &Constraint = &other.c.read_arc();
        lhs == rhs
    }

    fn equal_contents(&self, other: PyCArg) -> bool {
        self.c.read_arc().equal_contents(&other.c.read_arc())
    }
}

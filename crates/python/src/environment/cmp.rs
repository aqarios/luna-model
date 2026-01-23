use lunamodel_core::prelude::ContentEquality;
use lunamodel_unwind::unwindable;
use pyo3::pymethods;

use super::PyEnvironment;
use crate::unwind::unwind;

#[unwindable]
#[pymethods]
impl PyEnvironment {
    fn equal_contents(&self, other: &Self) -> bool {
        self.env.equal_contents(&other.env)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.env.id() == other.env.id() && self.env.equal_contents(&other.env)
    }
}

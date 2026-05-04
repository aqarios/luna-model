//! Equality and semantic-comparison helpers for Python environments.

use lunamodel_core::prelude::ContentEquality;
use lunamodel_unwind::*;
use pyo3::pymethods;

use crate::args::PyEnvArg;

use super::PyEnvironment;

#[unwindable]
#[pymethods]
impl PyEnvironment {
    fn equal_contents(&self, other: PyEnvArg) -> bool {
        self.env.equal_contents(&other.env)
    }

    fn __eq__(&self, other: PyEnvArg) -> bool {
        self.env.id() == other.env.id() && self.env.equal_contents(&other.env)
    }
}

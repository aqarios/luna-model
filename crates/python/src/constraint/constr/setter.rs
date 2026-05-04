//! In-place setters for the Python constraint wrapper.

use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyConstraint;

#[unwindable]
#[pymethods]
impl PyConstraint {
    #[setter]
    fn set_name(&self, name: String) -> PyResult<()> {
        Ok(self.c.write_arc().set_name(name)?)
    }
}

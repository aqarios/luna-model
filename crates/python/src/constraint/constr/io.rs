//! Display and debug formatting for Python constraints.

use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyConstraint;

#[unwindable]
#[pymethods]
impl PyConstraint {
    fn __str__(&self) -> String {
        format!("{}", self.c.read_arc().format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.c.read_arc().format(FormatOpt::Py))
    }
}

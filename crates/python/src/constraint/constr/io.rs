use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_unwind::unwindable;
use pyo3::pymethods;

use super::PyConstraint;
use crate::unwind::unwind;

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

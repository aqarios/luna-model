//! Display and debug formatting for Python models.

use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyModel;

#[unwindable]
#[pymethods]
impl PyModel {
    fn __str__(&self) -> String {
        format!("{}", self.m.read_arc().format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.m.read_arc().format(FormatOpt::Py))
    }
}

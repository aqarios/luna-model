use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_unwind::unwindable;
use pyo3::pymethods;

use super::PyBounds;
use crate::unwind::unwind;

#[unwindable]
#[pymethods]
impl PyBounds {
    fn __str__(&self) -> String {
        format!("{}", self.0.format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0.format(FormatOpt::Py))
    }
}

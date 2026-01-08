use lunamodel_io::{CustomFormat, FormatOpt};
use pyo3::pymethods;

use super::PyConstraint;

#[pymethods]
impl PyConstraint{
    fn __str__(&self) -> String {
        format!("{}", self.c.read_arc().format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.c.read_arc().format(FormatOpt::Py))
    }
}

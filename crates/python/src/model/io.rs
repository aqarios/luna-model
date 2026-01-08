use lunamodel_io::{CustomFormat, FormatOpt};
use pyo3::pymethods;

use super::PyModel;

#[pymethods]
impl PyModel {
    fn __str__(&self) -> String {
        format!("{}", self.m.read_arc().format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.m.read_arc().format(FormatOpt::Py))
    }
}


use lunamodel_io::{CustomFormat, FormatOpt};
use pyo3::pymethods;

use super::PySolution;

#[pymethods]
impl PySolution {
    fn __str__(&self) -> String {
        format!("{}", self.s.read_arc().format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.s.read_arc().format(FormatOpt::Py))
    }
}

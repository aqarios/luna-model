use lunamodel_io::{CustomFormat, FormatOpt};
use pyo3::pymethods;

use super::PyEnvironment;

#[pymethods]
impl PyEnvironment {
    fn __str__(&self) -> String {
        format!("{}", self.env.read_arc().format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.env.read_arc().format(FormatOpt::Py))
    }
}

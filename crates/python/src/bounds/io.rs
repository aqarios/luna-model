use lunamodel_io::{CustomFormat, FormatOpt};
use pyo3::pymethods;

use super::PyBounds;

#[pymethods]
impl PyBounds {
    fn __str__(&self) -> String {
        format!("{}", self.0.format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0.format(FormatOpt::Py))
    }
}

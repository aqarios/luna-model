use lunamodel_io::{CustomFormat, FormatOpt};
use pyo3::pymethods;

use super::PyVariable;

#[pymethods]
impl PyVariable {
    fn __str__(&self) -> String {
        format!("{}", self.v.format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.v.format(FormatOpt::Py))
    }
}

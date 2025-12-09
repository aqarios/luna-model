use lunamodel_io::{CustomFormat, FormatOpt};
use pyo3::pymethods;

use super::PyExpression;

#[pymethods]
impl PyExpression {
    fn __str__(&self) -> String {
        format!("{}", self.expr.format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.expr.format(FormatOpt::Py))
    }
}

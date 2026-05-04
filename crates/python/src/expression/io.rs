//! Display and debug formatting for Python expressions.

use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyExpression;

#[unwindable]
#[pymethods]
impl PyExpression {
    fn __str__(&self) -> String {
        format!("{}", self.expr.format(FormatOpt::Py))
    }

    // fn __repr__(&self) -> String {
    //     format!("{:?}", self.expr.format(FormatOpt::Py))
    // }
}

use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyModelSpecs;

#[unwindable]
#[pymethods]
impl PyModelSpecs {
    fn __str__(&self) -> String {
        format!("{}", self.s.format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.s.format(FormatOpt::Py))
    }
}

//! Display and debug formatting for Python variables.

use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyVariable;

#[unwindable]
#[pymethods]
impl PyVariable {
    fn __str__(&self) -> PyResult<String> {
        self.v.check_living()?;
        Ok(format!("{}", self.v.format(FormatOpt::Py)))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.v.check_living()?;
        Ok(format!("{:?}", self.v.format(FormatOpt::Py)))
    }
}

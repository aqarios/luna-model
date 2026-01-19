use lunamodel_io::{CustomFormat, FormatOpt};
use pyo3::{PyResult, pymethods};

use super::PyVariable;

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

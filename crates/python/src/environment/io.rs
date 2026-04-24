//! Display and debug formatting for Python environments.

use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_unwind::*;
use pyo3::pymethods;

use super::PyEnvironment;

#[unwindable]
#[pymethods]
impl PyEnvironment {
    fn __str__(&self) -> String {
        format!("{}", self.env.read_arc().format(FormatOpt::Py))
    }

    // fn __repr__(&self) -> String {
    //     format!("{:?}", self.env.read_arc().format(FormatOpt::Py))
    // }
}

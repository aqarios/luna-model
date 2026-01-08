use lunamodel_core::prelude::Environment;
use pyo3::{PyResult, pymethods};

use crate::PyEnvironment;

#[pymethods]
impl PyEnvironment {
    /// Initialize a new environment for variable construction.
    ///
    /// It is recommended to use this in a `with` statement to ensure proper scoping.
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(PyEnvironment::new(Environment::default()))
    }
}

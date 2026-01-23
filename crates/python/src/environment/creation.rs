use lunamodel_core::prelude::Environment;
use lunamodel_unwind::unwindable;
use pyo3::{Bound, PyResult, pymethods, types::PyType};

use crate::{PyEnvironment, environment::get_active_env, unwind::unwind};

#[unwindable]
#[pymethods]
impl PyEnvironment {
    /// Initialize a new environment for variable construction.
    ///
    /// It is recommended to use this in a `with` statement to ensure proper scoping.
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(PyEnvironment::new(Environment::default()))
    }

    #[classmethod]
    fn _from_ctx(_cls: &Bound<'_, PyType>) -> PyResult<PyEnvironment> {
        get_active_env()
    }
}

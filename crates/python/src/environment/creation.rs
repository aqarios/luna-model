//! Constructors for the Python `Environment` wrapper.

use lunamodel_core::prelude::Environment;
use lunamodel_unwind::*;
use pyo3::{Bound, PyResult, pymethods, types::PyType};

use crate::{PyEnvironment, environment::get_active_env};

#[unwindable]
#[pymethods]
impl PyEnvironment {
    /// Initialize a new environment for variable construction.
    ///
    /// The returned wrapper owns a fresh Rust [`Environment`]. In Python-facing
    /// code it is typically used as a context manager so nested constructors can
    /// resolve the active environment implicitly.
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(PyEnvironment::new(Environment::default()))
    }

    /// Resolve the currently active environment from thread-local context.
    ///
    /// This exists mainly so other Python wrapper constructors can share one
    /// implementation path for explicit and implicit environment lookup.
    #[classmethod]
    fn _from_ctx(_cls: &Bound<'_, PyType>) -> PyResult<PyEnvironment> {
        get_active_env()
    }
}

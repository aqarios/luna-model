//! Python environment wrapper and active-environment context handling.
mod access;
mod cmp;
mod context;
mod creation;
mod io;
mod ser;

use lunamodel_core::{ArcEnv, Environment};
use lunamodel_error::py::PyNoActiveEnvironmentFoundError;
use pyo3::{PyErr, PyResult, pyclass};
use std::cell::RefCell;

thread_local! {
    pub(crate) static ACTIVE_ENV: RefCell<Option<PyEnvironment>> = const { RefCell::new(None) };
}

/// Returns the currently active Python environment or a Python error if none exists.
pub(crate) fn get_active_env() -> PyResult<PyEnvironment> {
    ACTIVE_ENV.with(|current| {
        current
            .borrow()
            .clone()
            .ok_or_else(|| PyNoActiveEnvironmentFoundError::new_err("no active environment found."))
    })
}

// #[pyclass(name = "Environment", module = "luna_model._core")]
#[pyclass(from_py_object)]
#[derive(Debug)]
#[repr(C)]
pub struct PyEnvironment {
    /// Shared core environment handle.
    pub env: ArcEnv,
}

impl Clone for PyEnvironment {
    /// Clones the shared environment handle.
    fn clone(&self) -> Self {
        PyEnvironment {
            env: self.env.clone(),
        }
    }
}

impl From<Environment> for PyEnvironment {
    /// Wraps an owned core environment for Python.
    fn from(env: Environment) -> Self {
        Self { env: env.into() }
    }
}

impl From<ArcEnv> for PyEnvironment {
    /// Wraps a shared core environment for Python.
    fn from(env: ArcEnv) -> Self {
        Self { env }
    }
}

impl TryFrom<Option<PyEnvironment>> for PyEnvironment {
    type Error = PyErr;
    /// Resolves an optional environment, falling back to the active context.
    fn try_from(value: Option<PyEnvironment>) -> Result<Self, Self::Error> {
        match value {
            Some(env) => Ok(env.clone()),
            None => get_active_env(),
        }
    }
}

impl PyEnvironment {
    /// Creates a Python wrapper from an owned core environment.
    pub(crate) fn new(e: Environment) -> Self {
        PyEnvironment { env: e.into() }
    }
}

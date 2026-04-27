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

pub(crate) fn get_active_env() -> PyResult<PyEnvironment> {
    ACTIVE_ENV.with(|current| {
        current
            .borrow()
            .clone()
            .ok_or_else(|| PyNoActiveEnvironmentFoundError::new_err("no active environment found."))
    })
}

// #[pyclass(name = "Environment", module = "luna_model._core")]
#[pyclass]
#[derive(Debug)]
#[repr(C)]
pub struct PyEnvironment {
    pub env: ArcEnv,
}

impl Clone for PyEnvironment {
    fn clone(&self) -> Self {
        PyEnvironment {
            env: self.env.clone(),
        }
    }
}

impl From<Environment> for PyEnvironment {
    fn from(env: Environment) -> Self {
        Self { env: env.into() }
    }
}

impl From<ArcEnv> for PyEnvironment {
    fn from(env: ArcEnv) -> Self {
        Self { env }
    }
}

impl TryFrom<Option<PyEnvironment>> for PyEnvironment {
    type Error = PyErr;
    fn try_from(value: Option<PyEnvironment>) -> Result<Self, Self::Error> {
        match value {
            Some(env) => Ok(env.clone()),
            None => get_active_env(),
        }
    }
}

impl PyEnvironment {
    pub(crate) fn new(e: Environment) -> Self {
        PyEnvironment { env: e.into() }
    }
}

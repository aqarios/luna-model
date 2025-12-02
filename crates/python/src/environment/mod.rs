mod context;
mod general;
// mod ser;

use lunamodel_core::{ArcEnv, Environment};
use pyo3::{PyErr, PyResult, pyclass};
use std::cell::RefCell;

use crate::exceptions::NoActiveEnvironmentFoundError;

thread_local! {
    pub(crate) static ACTIVE_ENV: RefCell<Option<PyEnvironment>> = RefCell::new(None);
}

pub(crate) fn get_active_env() -> PyResult<PyEnvironment> {
    Ok(ACTIVE_ENV.with(|current| {
        current
            .borrow()
            .clone()
            .ok_or_else(|| NoActiveEnvironmentFoundError::new_err("no active environment found."))
    })?)
}

// #[pyclass(name = "Environment", module = "luna_model._core")]
#[pyclass]
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

// impl From<Option<PyEnvironment>> for PyEnvironment {
//     fn from(value: Option<PyEnvironment>) -> Self {
//         match value {
//             Some(env) => env.clone(),
//             None => get_active_env()?,
//         }
//     }
// }

// impl From<PyEnvironment> for Environment {
//     fn from(value: PyEnvironment) -> Self {
//         value.e.clone().into_inner()
//     }
// }
//

impl PyEnvironment {
    pub(crate) fn new(e: Environment) -> Self {
        PyEnvironment { env: e.into() }
    }
}

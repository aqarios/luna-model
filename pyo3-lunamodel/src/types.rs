use std::sync::Arc;

use lunamodel_core::prelude::*;
use parking_lot::RwLock;
use pyo3::{FromPyObject, PyErr, types::PyAnyMethods};

#[repr(transparent)]
#[derive(Debug, Clone)]
/// A wrapper around a [`ArcEnv`] that can be converted to and from python with `pyo3`.
pub struct PyEnvironment(pub ArcEnv);

impl<'a, 'py> FromPyObject<'a, 'py> for PyEnvironment {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        // check if it is the wrapper type or the PyEnvironment type from the crate.
        let raw_ptr: usize = obj.call_method0("into_raw_ptr")?.extract()?;
        let arc: ArcEnv = ArcEnv {
            env: unsafe { Arc::from_raw(raw_ptr as *const RwLock<Environment>) },
        };
        Ok(PyEnvironment(arc))
    }
}

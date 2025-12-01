use std::sync::Arc;

use lunamodel_core::{ArcEnv, Environment};
use parking_lot::RwLock;
use pyo3::prelude::*;

use crate::PyEnvironment;

#[pymethods]
impl PyEnvironment {
    /// Investigate if PyCapusle is better suited.
    pub fn into_raw_ptr(&self) -> usize {
        let cloned = Arc::clone(&self.env);
        Arc::into_raw(cloned) as usize
    }

    #[staticmethod]
    pub fn from_raw_ptr(py: Python, raw_ptr: usize) -> PyResult<Py<PyAny>> {
        let arc: ArcEnv = ArcEnv {
            env: unsafe { Arc::from_raw(raw_ptr as *const RwLock<Environment>) },
        };
        let pyenv = Py::new(py, PyEnvironment { env: arc }).map_err(|e| PyErr::from(e))?;
        Ok(pyenv.into_any())
    }
}

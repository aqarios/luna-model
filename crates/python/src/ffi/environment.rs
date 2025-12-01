use lunamodel_core::ArcEnv;
use pyo3::prelude::*;

use crate::PyEnvironment;

#[pymethods]
impl PyEnvironment {
    /// Investigate if PyCapusle is better suited.
    pub fn _into_raw_ptr(&self) -> usize {
        self.env.into_raw_ptr().into()
    }

    #[staticmethod]
    pub fn _from_raw_ptr(raw_ptr: usize) -> PyEnvironment {
        let arc: ArcEnv = ArcEnv::from_raw_ptr(raw_ptr.into());
        PyEnvironment { env: arc }
    }
}

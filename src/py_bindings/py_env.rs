use crate::core::Environment;

use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

#[pyclass(name = "Environment")]
#[derive(Deref, DerefMut)]
pub struct PyEnvironment(Environment);

#[pymethods]
impl PyEnvironment {
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(PyEnvironment(Environment::new()))
    }
}

use lunamodel_types::Vtype;
use pyo3::prelude::*;

use crate::{PyEnvironment, PyVariable};

#[pymethods]
impl PyVariable {
    #[new]
    #[pyo3(signature=(name, vtype, env=None))]
    pub fn py_new(name: String, vtype: Vtype, env: Option<PyEnvironment>) -> PyResult<Self> {
        let mut penv: PyEnvironment = env.try_into()?;
        let vref = penv.env.insert(&name, vtype, None).unwrap(); // todo@jflxb map
        // error using ?
        Ok(PyVariable::new(vref))
    }
}

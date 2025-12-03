use lunamodel_types::Vtype;
use pyo3::prelude::*;

use crate::{bounds::PyBounds, environment::PyEnvironment, expression::PyExpression};
use super::PyVariable

#[pymethods]
impl PyVariable {
    #[new]
    #[pyo3(signature=(name, vtype, bounds=None, env=None))]
    fn py_new(
        name: String,
        vtype: Vtype,
        bounds: Option<PyBounds>,
        env: Option<PyEnvironment>,
    ) -> PyResult<Self> {
        let mut penv: PyEnvironment = env.try_into()?;
        let vref = penv.env.insert(&name, vtype, bounds.map(|b| b.0))?;
        Ok(PyVariable::new(vref))
    }
}

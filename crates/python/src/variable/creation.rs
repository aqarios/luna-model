use lunamodel_error::LunaModelError;
use lunamodel_types::Vtype;
use lunamodel_unwind::*;
use pyo3::prelude::*;

use super::PyVariable;
use crate::{bounds::PyBounds, environment::PyEnvironment};

#[unwindable]
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
        if vtype == Vtype::InvertedBinary {
            return Err(LunaModelError::UnsupportedOperation(
                "cannot create an inverted binary variable directly. Use the '.inv` method or the '~' operator.".into(),
            ))?;
        };
        let mut penv: PyEnvironment = env.try_into()?;
        let vref = penv.env.insert(&name, vtype, bounds.map(|b| b.0.into()))?;
        Ok(PyVariable::new(vref))
    }
}

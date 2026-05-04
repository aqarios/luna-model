//! Constructors for Python variables.

use lunamodel_error::LunaModelError;
use lunamodel_unwind::*;
use pyo3::prelude::*;

use super::PyVariable;
use crate::{
    args::{PyBoundsArg, PyEnvArg},
    bounds::BoundsContent,
    environment::PyEnvironment,
    types::PyVtype,
};

#[unwindable]
#[pymethods]
impl PyVariable {
    #[new]
    #[pyo3(signature=(name, vtype, bounds=None, env=None))]
    fn py_new(
        name: String,
        vtype: PyVtype,
        bounds: Option<PyBoundsArg>,
        env: Option<PyEnvArg>,
    ) -> PyResult<Self> {
        if vtype == PyVtype::InvertedBinary {
            return Err(LunaModelError::UnsupportedOperation(
                "cannot create an inverted binary variable directly. Use the '.inv` method or the '~' operator.".into(),
            ))?;
        };
        let mut penv: PyEnvironment = env.try_into()?;
        let vref = penv.env.insert(
            &name,
            vtype.into(),
            bounds.map(|b| {
                let bc: &BoundsContent = &b.0.read_arc();
                bc.clone().into()
            }),
        )?;
        Ok(PyVariable::new(vref))
    }
}

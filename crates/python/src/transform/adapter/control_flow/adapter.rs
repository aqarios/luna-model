use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::{ControlFlowPass, ControlFlowPlan, PassContext};
use pyo3::{Py, PyErr, Python, types::PyAnyMethods};

use super::{PyControlFlowPass, PyControlFlowPlan};
use crate::{
    PyModel,
    transform::{PyPassContext, utils::map_pyerr},
};

pub struct PyControlFlowPassAdapter {
    inner: Py<PyControlFlowPass>,
    name: String,
    requires: Vec<String>,
    invalidates: Vec<String>,
    provides: Vec<String>,
}

impl PyControlFlowPassAdapter {
    pub fn new(py: Python, inner: Py<PyControlFlowPass>) -> LunaModelResult<Self> {
        let name: String = inner
            .call_method0(py, "name")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;
        let requires: Vec<String> = inner
            .call_method0(py, "requires")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;
        let invalidates: Vec<String> = inner
            .call_method0(py, "invalidates")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;
        let provides: Vec<String> = inner
            .call_method0(py, "provides")
            .map_err(map_pyerr)?
            .extract(py)
            .map_err(map_pyerr)?;
        Ok(Self {
            inner,
            name,
            requires,
            invalidates,
            provides,
        })
    }
}

impl ControlFlowPass for PyControlFlowPassAdapter {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self, model: &Model, ctx: &PassContext) -> LunaModelResult<ControlFlowPlan> {
        Python::attach(|py| {
            let obj = self.inner.bind(py);

            let res = obj
                .call_method1(
                    "_run",
                    (
                        PyModel::from(model.clone()),
                        PyPassContext::from(ctx.manager().clone()),
                    ),
                )
                .map_err(map_pyerr)?;
            let plan: PyControlFlowPlan = res.extract().map_err(PyErr::from).map_err(map_pyerr)?;
            Ok(plan.0)
        })
    }

    fn requires(&self) -> &[String] {
        &self.requires
    }

    fn provides(&self) -> &[String] {
        &self.provides
    }

    fn invalidates(&self) -> &[String] {
        &self.invalidates
    }
}

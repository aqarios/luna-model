use std::sync::Arc;

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;
use lunamodel_python_macros::pycontrolflow;
use lunamodel_transform::control_flow::{ConditionPredicate, IfElsePass};
use lunamodel_transpiler::{PassContext};
use pyo3::{Py, PyAny, PyResult, Python, pymethods};

use crate::{
    PyModel,
    transform::{
        PyPassContext,
        utils::{PipelineOrPassVec, map_pyerr},
    },
};

#[pycontrolflow]
pub struct PyIfElsePass(pub IfElsePass);

#[pymethods]
impl PyIfElsePass {
    #[new]
    fn new(
        py: Python,
        condition: Py<PyAny>,
        then: PipelineOrPassVec,
        otherwise: PipelineOrPassVec,
        name: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self(IfElsePass::new(
            Arc::new(PyCondition(condition)),
            then.to_steps(py)?,
            otherwise.to_steps(py)?,
            name,
        )))
    }
}

struct PyCondition(Py<PyAny>);
impl ConditionPredicate for PyCondition {
    fn eval(&self, model: &Model, ctx: &PassContext) -> LunaModelResult<bool> {
        Python::attach(|py| {
            let res = self
                .0
                .call1(
                    py,
                    (
                        PyModel::from(model.clone()),
                        PyPassContext::from(ctx.manager().clone()),
                    ),
                )
                .map_err(map_pyerr)?;
            res.extract::<bool>(py).map_err(map_pyerr)
        })
    }
}

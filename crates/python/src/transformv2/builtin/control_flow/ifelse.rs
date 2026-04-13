use std::sync::Arc;

use lunamodel_core::Model;
use lunamodel_error::LunaModelResult;
use lunamodel_transformv2::{ConditionPredicate, IfElsePass};
use lunamodel_transpiler::{ControlFlowPass, PassContext};
use pyo3::{Py, PyAny, PyResult, Python, pyclass, pymethods};

use crate::{
    PyModel,
    transformv2::{
        PyControlFlowPlan, PyPassContext,
        utils::{PipelineOrPassVec, map_pyerr},
    },
};

#[pyclass(subclass)]
pub struct PyIfElsePass(IfElsePass);

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

    fn name(&self) -> String {
        self.0.name().to_string()
    }

    fn run(&self, model: &PyModel, ctx: &PyPassContext) -> LunaModelResult<PyControlFlowPlan> {
        Ok(self.0.run(&model.m.read_arc(), &ctx.into())?.into())
    }

    fn requires(&self) -> Vec<String> {
        self.0.requires().to_vec()
    }

    fn provides(&self) -> Vec<String> {
        self.0.provides().to_vec()
    }

    fn invalidates(&self) -> Vec<String> {
        self.0.invalidates().to_vec()
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

impl PyIfElsePass {
    pub fn to_rs(&self) -> IfElsePass {
        self.0.clone()
    }
}

use pyo3::prelude::*;

use crate::transformations::{
    base_passes::Pass,
    passes::ifelse::{Condition, IfElsePass},
};

use super::{py_pipeline_adapter::PyPipelineAdapter, PyPipeline};

#[pyclass(unsendable, name = "IfElsePass")]
#[derive(Debug, Clone)]
pub struct PyIfElsePass(pub IfElsePass);

#[pymethods]
impl PyIfElsePass {
    #[new]
    #[pyo3(signature = (requires, condition, then, otherwise, name=None))]
    pub fn py_new(
        requires: Vec<String>,
        condition: Condition,
        then: Py<PyPipeline>,
        otherwise: Py<PyPipeline>,
        name: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self(IfElsePass::new(
            requires,
            condition,
            Box::new(PyPipelineAdapter::new(then)?),
            Box::new(PyPipelineAdapter::new(otherwise)?),
            name,
        )))
    }
}

impl PyIfElsePass {
    pub fn as_pass(self) -> PyResult<Pass> {
        Ok(Pass::IfElse(self.0))
    }
}

use pyo3::prelude::*;

use crate::transformations::{
    base_passes::Pass,
    passes::ifelse::{Condition, IfElsePass},
};

use super::PyPipeline;

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
        then: &PyPipeline,
        otherwise: &PyPipeline,
        name: Option<String>,
    ) -> Self {
        Self(IfElsePass::new(
            requires,
            condition,
            then.0.clone(),
            otherwise.0.clone(),
            name,
        ))
    }
}

impl PyIfElsePass {
    pub fn as_pass(self) -> PyResult<Pass> {
        Ok(Pass::IfElse(self.0))
    }
}

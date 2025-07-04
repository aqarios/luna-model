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
    pub fn py_new(
        required: Vec<String>,
        condition: Condition,
        then: &PyPipeline,
        otherwise: &PyPipeline,
    ) -> Self {
        Self(IfElsePass::new(
            required,
            condition,
            then.0.clone(),
            otherwise.0.clone(),
        ))
    }
}

impl PyIfElsePass {
    pub fn as_pass(self) -> PyResult<Pass> {
        Ok(Pass::IfElse(self.0))
    }
}

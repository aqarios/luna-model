use std::sync::Arc;

use lunamodel_transpiler::PipelineStep;
use pyo3::{FromPyObject, Py, PyResult, Python};

use super::builtin::transformation::PyIntegerToBinaryPass;
use crate::transformv2::{
    PyTransformationPassAdapter,
    adapter::{PyAnalysisPass, PyAnalysisPassAdapter, PyTransformationPass},
};

#[derive(FromPyObject)]
pub enum PyPass {
    IntToBin(Py<PyIntegerToBinaryPass>),
    CustomTransformation(Py<PyTransformationPass>),
    CustomAnalysis(Py<PyAnalysisPass>),
}

impl PyPass {
    pub fn to_step(&self, py: Python) -> PyResult<PipelineStep> {
        match self {
            Self::IntToBin(p) => Ok(PipelineStep::Transform(Arc::new(p.borrow(py).to_rs()))),
            Self::CustomTransformation(p) => Ok(PipelineStep::Transform(Arc::new(
                PyTransformationPassAdapter::new(py, p.clone_ref(py))?,
            ))),
            Self::CustomAnalysis(p) => Ok(PipelineStep::Analysis(Arc::new(
                PyAnalysisPassAdapter::new(py, p.clone_ref(py))?,
            ))),
        }
        // self.inner.clone()
    }
}

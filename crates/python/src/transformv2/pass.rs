use std::sync::Arc;

use lunamodel_transpiler::PipelineStep;
use pyo3::{FromPyObject, Py, PyResult, Python};

use super::builtin::{
    analysis::{
        PyCheckModelSpecsAnalysis, PyMaxBiasAnalysis, PyMinValueForConstraintAnalysis,
        PySpecsAnalysis,
    },
    transformation::PyIntegerToBinaryPass,
};
use crate::transformv2::{
    PyTransformationPassAdapter,
    adapter::{PyAnalysisPass, PyAnalysisPassAdapter, PyTransformationPass},
};

#[derive(FromPyObject)]
pub enum PyPass {
    // analysis
    CheckSpecs(Py<PyCheckModelSpecsAnalysis>),
    MaxBias(Py<PyMaxBiasAnalysis>),
    MinValInConstr(Py<PyMinValueForConstraintAnalysis>),
    Specs(Py<PySpecsAnalysis>),
    // transformation
    IntToBin(Py<PyIntegerToBinaryPass>),
    // custom from python.
    CustomTransformation(Py<PyTransformationPass>),
    CustomAnalysis(Py<PyAnalysisPass>),
}

impl PyPass {
    pub fn to_step(&self, py: Python) -> PyResult<PipelineStep> {
        match self {
            // analysis
            Self::CheckSpecs(p) => Ok(PipelineStep::Analysis(Arc::new(p.borrow(py).to_rs()))),
            Self::MaxBias(p) => Ok(PipelineStep::Analysis(Arc::new(p.borrow(py).to_rs()))),
            Self::MinValInConstr(p) => Ok(PipelineStep::Analysis(Arc::new(p.borrow(py).to_rs()))),
            Self::Specs(p) => Ok(PipelineStep::Analysis(Arc::new(p.borrow(py).to_rs()))),
            // transformation
            Self::IntToBin(p) => Ok(PipelineStep::Transform(Arc::new(p.borrow(py).to_rs()))),
            // custom from python.
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

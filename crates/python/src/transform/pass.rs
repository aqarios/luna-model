use lunamodel_transform::Pass;
use pyo3::{FromPyObject, IntoPyObject, Py, PyResult, Python};

use super::{
    PyIfElsePass, PyPipeline,
    adapters::{PyAnalysisPassAdapter, PyPipelineAdapter, PyTransformationPassAdapter},
    interfaces::{
        PyAnalysisPass,
        PyTransformationPass,
        // PyMetaAnalaysisPass,
    },
};

// #[pyclass(unsendable)]
#[derive(Debug, FromPyObject, IntoPyObject)]
pub enum PyPass {
    Transformation(Py<PyTransformationPass>),
    Analysis(Py<PyAnalysisPass>),
    // MetaAnalysis(PyMetaAnalaysisPass),
    IfElse(PyIfElsePass),
    Pipeline(Py<PyPipeline>),
}

impl PyPass {
    pub fn as_pass(&self) -> PyResult<Pass> {
        match &self {
            Self::Transformation(p) => Ok(Pass::Transformation(Box::new(
                PyTransformationPassAdapter::new(Python::attach(|py| p.clone_ref(py)))?,
            ))),
            Self::Analysis(p) => Ok(Pass::Analysis(Box::new(PyAnalysisPassAdapter::new(
                Python::attach(|py| p.clone_ref(py)),
            )?))),
            Self::IfElse(p) => Ok(Pass::IfElse(p.0.clone())),
            Self::Pipeline(p) => Ok(Pass::Pipeline(Box::new(PyPipelineAdapter::new(
                Python::attach(|py| p.clone_ref(py)),
            )?))),
        }
    }
}

impl Clone for PyPass {
    fn clone(&self) -> Self {
        match &self {
            Self::Transformation(p) => Self::Transformation(Python::attach(|py| p.clone_ref(py))),
            Self::Analysis(p) => Self::Analysis(Python::attach(|py| p.clone_ref(py))),
            Self::IfElse(p) => Self::IfElse(p.clone()),
            Self::Pipeline(p) => Self::Pipeline(Python::attach(|py| p.clone_ref(py))),
        }
    }
}

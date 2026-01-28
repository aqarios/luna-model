use lunamodel_error::LunaModelResult;
use lunamodel_transform::{Pass, TransformationPass};
use pyo3::{FromPyObject, IntoPyObject, Py, PyResult, Python};

use crate::transform::{
    PyChangeSensePass, adapters::PyMetaAnalysisPassAdapter, interfaces::PyMetaAnalysisPass,
};

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
    MetaAnalysis(Py<PyMetaAnalysisPass>),
    IfElse(PyIfElsePass),
    Pipeline(Py<PyPipeline>),
    // trial
    Cs(PyChangeSensePass),
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
            Self::MetaAnalysis(p) => Ok(Pass::MetaAnalysis(Box::new(
                PyMetaAnalysisPassAdapter::new(Python::attach(|py| p.clone_ref(py)))?,
            ))),
            Self::IfElse(p) => Ok(Pass::IfElse(p.0.clone())),
            Self::Pipeline(p) => Ok(Pass::Pipeline(Box::new(PyPipelineAdapter::new(
                Python::attach(|py| p.clone_ref(py)),
            )?))),
            // trial
            Self::Cs(c) => Ok(Pass::Transformation(Box::new(c.p.clone()))),
        }
    }

    pub fn from_pass(pass: &Pass) -> PyPass {
        match pass {
            Pass::Transformation(t) => todo!(),
            Pass::Analysis(a) => todo!(),
            Pass::IfElse(e) => todo!(),
            Pass::Pipeline(p) => todo!(),
            Pass::MetaAnalysis(m) => todo!(),
        }
    }
}

impl Clone for PyPass {
    fn clone(&self) -> Self {
        match &self {
            Self::Transformation(p) => Self::Transformation(Python::attach(|py| p.clone_ref(py))),
            Self::Analysis(p) => Self::Analysis(Python::attach(|py| p.clone_ref(py))),
            Self::MetaAnalysis(p) => Self::MetaAnalysis(Python::attach(|py| p.clone_ref(py))),
            Self::IfElse(p) => Self::IfElse(p.clone()),
            Self::Pipeline(p) => Self::Pipeline(Python::attach(|py| p.clone_ref(py))),
        }
    }
}

pub trait AsPyPass {
    fn as_pypass(&self) -> PyPass;
}

use lunamodel_transform::Pass;
use pyo3::{FromPyObject, IntoPyObject, Py, PyResult, Python};

use crate::transform::{adapters::PyMetaAnalysisPassAdapter, interfaces::PyMetaAnalysisPass};

use super::{
    PyIfElsePass, PyPipeline,
    adapters::{PyAnalysisPassAdapter, PyPipelineAdapter, PyTransformationPassAdapter},
    interfaces::{PyAnalysisPass, PyTransformationPass},
};

#[derive(Debug, FromPyObject, IntoPyObject)]
pub enum PyPass {
    Transformation(Py<PyTransformationPass>),
    Analysis(Py<PyAnalysisPass>),
    MetaAnalysis(Py<PyMetaAnalysisPass>),
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
            Self::MetaAnalysis(p) => Ok(Pass::MetaAnalysis(Box::new(
                PyMetaAnalysisPassAdapter::new(Python::attach(|py| p.clone_ref(py)))?,
            ))),
            Self::IfElse(p) => Ok(Pass::IfElse(p.0.clone())),
            Self::Pipeline(p) => Ok(Pass::Pipeline(Box::new(PyPipelineAdapter::new(
                Python::attach(|py| p.clone_ref(py)),
            )?))),
        }
    }

    pub fn from_pass(pass: &Pass) -> PyResult<PyPass> {
        Python::attach(|py| match pass {
            Pass::Transformation(t) => {
                if let Some(adapter) = t
                    .as_any()
                    .and_then(|a| a.downcast_ref::<PyTransformationPassAdapter>())
                {
                    Ok(PyPass::Transformation(adapter.inner.clone_ref(py)))
                } else {
                    Err(pyo3::exceptions::PyTypeError::new_err(
                        "Cannot convert native Rust TransformationPass to PyPass",
                    ))
                }
            }
            Pass::Analysis(a) => {
                if let Some(adapter) = a
                    .as_any()
                    .and_then(|a| a.downcast_ref::<PyAnalysisPassAdapter>())
                {
                    Ok(PyPass::Analysis(adapter.inner.clone_ref(py)))
                } else {
                    Err(pyo3::exceptions::PyTypeError::new_err(
                        "Cannot convert native Rust AnalysisPass to PyPass",
                    ))
                }
            }
            Pass::IfElse(e) => Ok(PyPass::IfElse(PyIfElsePass(e.clone()))),
            Pass::Pipeline(p) => {
                if let Some(adapter) = p
                    .as_any()
                    .and_then(|a| a.downcast_ref::<PyPipelineAdapter>())
                {
                    Ok(PyPass::Pipeline(adapter.inner.clone_ref(py)))
                } else {
                    Err(pyo3::exceptions::PyTypeError::new_err(
                        "Cannot convert native Rust Pipeline to PyPass",
                    ))
                }
            }
            Pass::MetaAnalysis(m) => {
                if let Some(adapter) = m
                    .as_any()
                    .and_then(|a| a.downcast_ref::<PyMetaAnalysisPassAdapter>())
                {
                    Ok(PyPass::MetaAnalysis(adapter.inner.clone_ref(py)))
                } else {
                    Err(pyo3::exceptions::PyTypeError::new_err(
                        "Cannot convert native Rust MetaAnalysisPass to PyPass",
                    ))
                }
            }
        })
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

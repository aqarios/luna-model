use lunamodel_transform::Pass;
use pyo3::{FromPyObject, Py, PyResult};

use super::{
    adapters::{PyAnalysisPassAdapter, PyTransformationPassAdapter},
    interfaces::{
        PyAnalysisPass,
        PyTransformationPass,
        // PyIfElsePass,
        // PyMetaAnalaysisPass,
        // PyPipeline,
    },
};

// #[pyclass(unsendable)]
#[derive(Debug, FromPyObject)]
pub enum PyPass {
    Transformation(Py<PyTransformationPass>),
    Analysis(Py<PyAnalysisPass>),
    // MetaAnalysis(PyMetaAnalaysisPass),
    // IfElse(PyIfElsePass),
    // Pipeline(PyPipeline),
}

impl PyPass {
    pub fn as_pass(self) -> PyResult<Pass> {
        match self {
            Self::Transformation(p) => Ok(Pass::Transformation(Box::new(
                PyTransformationPassAdapter::new(p)?,
            ))),
            Self::Analysis(p) => Ok(Pass::Analysis(Box::new(PyAnalysisPassAdapter::new(p)?))),
        }
    }
}

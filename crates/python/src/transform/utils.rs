//! Internal helper traits for Python transformation wrappers.

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transpiler::PipelineStep;
use pyo3::{FromPyObject, Py, PyAny, PyErr, PyResult, Python};

use crate::transform::{pass::PyPass, pipeline::PyPipeline};

pub fn map_pyerr(err: PyErr) -> LunaModelError {
    LunaModelError::WithCause(
        Box::new(LunaModelError::Internal(err.to_string().into())),
        err.into(),
    )
}

#[derive(FromPyObject)]
pub enum PipelineOrPassVec {
    Pipeline(PyPipeline),
    PassVec(Vec<PyPass>),
}

impl PipelineOrPassVec {
    pub fn to_steps(&self, py: Python) -> PyResult<Vec<PipelineStep>> {
        match self {
            Self::Pipeline(p) => Ok(p.steps().to_vec()),
            Self::PassVec(v) => v.iter().map(|p| p.to_step(py)).collect::<PyResult<_>>(),
        }
    }
}

pub trait FromSteps {
    /// Convert a slice of pipeline steps into Python-facing PyPass.
    fn to_pypasses(&self, py: Python) -> LunaModelResult<Vec<Py<PyAny>>>;
}

impl FromSteps for &[PipelineStep] {
    /// Convert all steps in this slice into Python-facing views.
    fn to_pypasses(&self, py: Python) -> LunaModelResult<Vec<Py<PyAny>>> {
        self.iter().map(|s| PyPass::from_step(py, s)).collect()
    }
}

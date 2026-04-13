use lunamodel_error::LunaModelError;
use lunamodel_transpiler::PipelineStep;
use pyo3::{FromPyObject, PyErr, PyResult, Python};

use crate::transformv2::{pass::PyPass, pipeline::PyPipeline};

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
    pub fn to_steps(self, py: Python) -> PyResult<Vec<PipelineStep>> {
        match self {
            Self::Pipeline(p) => Ok(p.steps()),
            Self::PassVec(v) => v
                .into_iter()
                .map(|p| p.to_step(py))
                .collect::<PyResult<_>>(),
        }
    }
}

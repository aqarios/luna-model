use pyo3::{FromPyObject, Py, PyResult};

use crate::{
    py_bindings::py_transformations::{PyAnalysisPass, PyTransformationPass},
    transformations::{base_passes::Pass, passes::{change_sense::PyChangeSensePass, max_bias::PyMaxBiasAnalysis}},
};

use super::py_pass_base::PyPass;

#[derive(FromPyObject)]
pub enum AnyPass {
    ChangeSense(PyChangeSensePass),
    MaxBias(PyMaxBiasAnalysis),
    PyTransformationPass(Py<PyTransformationPass>),
    PyAnalysisPass(Py<PyAnalysisPass>),
}

impl AnyPass {
    pub fn as_pass(self) -> PyResult<Pass> {
        Ok(match self {
            Self::ChangeSense(x) => x.as_pass()?,
            Self::MaxBias(x) => x.as_pass()?,
            Self::PyTransformationPass(x) => x.as_pass()?,
            Self::PyAnalysisPass(x) => x.as_pass()?,
        })
    }
}

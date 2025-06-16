use pyo3::FromPyObject;

use crate::transformations::base_passes::Pass;

use super::{
    py_change_sense::PyChangeSensePass, py_max_bias::PyMaxBiasAnalysis, py_pass_base::PyPass,
};

#[derive(FromPyObject)]
pub enum AnyPass {
    ChangeSense(PyChangeSensePass),
    MaxBias(PyMaxBiasAnalysis),
}

impl AnyPass {
    pub fn as_pass(self) -> Pass {
        match self {
            Self::ChangeSense(x) => x.as_pass(),
            Self::MaxBias(x) => x.as_pass(),
        }
    }
}

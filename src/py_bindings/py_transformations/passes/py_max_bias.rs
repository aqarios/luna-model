use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

use crate::transformations::{
    base_passes::{BasePass, Pass},
    passes::max_bias::MaxBiasAnalysis,
};

use super::py_pass_base::PyPass;

// TODO: Docs
#[pyclass(name = "MaxBiasAnalysis")]
#[derive(Deref, DerefMut, Clone)]
pub struct PyMaxBiasAnalysis(MaxBiasAnalysis);

#[pymethods]
impl PyMaxBiasAnalysis {
    #[new]
    pub fn py_init() -> Self {
        PyMaxBiasAnalysis(MaxBiasAnalysis {})
    }

    #[getter]
    pub fn get_name(&self) -> String {
        self.name().to_owned()
    }

    #[getter]
    pub fn get_requires(&self) -> Vec<String> {
        self.requires().iter().map(|&x| x.to_owned()).collect()
    }
}

impl PyPass for PyMaxBiasAnalysis {
    fn as_pass(self) -> Pass {
        Pass::Analysis(Box::new(self.0))
    }
}

use lunamodel_transformv2::analysis::{MaxBias, MaxBiasAnalysis};
use lunamodel_transpiler::AnalysisPass;
use pyo3::{Bound, PyResult, pyclass, pymethods, types::PyType};

use crate::{PyModel, transformv2::PyPassContext};

#[pyclass]
pub struct PyMaxBias(pub MaxBias);

#[pymethods]
impl PyMaxBias {
    #[getter]
    fn val(&self) -> f64 {
        self.0.val
    }
}

#[pyclass(subclass)]
#[derive(Default)]
pub struct PyMaxBiasAnalysis(pub MaxBiasAnalysis);

#[pymethods]
impl PyMaxBiasAnalysis {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[classmethod]
    fn provides(_cls: &Bound<'_, PyType>) -> String {
        MaxBiasAnalysis::PROVIDES.to_string()
    }

    fn name(&self) -> String {
        self.0.name().to_string()
    }

    fn run(&self, model: PyModel, ctx: &PyPassContext) -> PyResult<PyMaxBias> {
        Ok(PyMaxBias(self.0.run(&model.0.m.read_arc(), &ctx.into())?))
    }

    fn requires(&self) -> Vec<String> {
        self.0.requires().to_vec()
    }

    fn __str__(&self) -> String {
        self.0.display()
    }
}

impl PyMaxBiasAnalysis {
    pub fn to_rs(&self) -> MaxBiasAnalysis {
        self.0.clone()
    }
}

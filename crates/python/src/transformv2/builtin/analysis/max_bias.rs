use lunamodel_transformv2::analysis::{MaxBias, MaxBiasAnalysis};
use lunamodel_transpiler::AnalysisPass;
use pyo3::{Bound, pyclass, pymethods, types::PyType};

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
}

impl PyMaxBiasAnalysis {
    pub fn to_rs(&self) -> MaxBiasAnalysis {
        self.0.clone()
    }
}

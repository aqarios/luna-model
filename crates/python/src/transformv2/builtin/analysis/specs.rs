use lunamodel_transformv2::analysis::SpecsAnalysis;
use lunamodel_transpiler::AnalysisPass;
use pyo3::{Bound, pyclass, pymethods, types::PyType};

#[pyclass(subclass)]
#[derive(Default)]
pub struct PySpecsAnalysis(pub SpecsAnalysis);

#[pymethods]
impl PySpecsAnalysis {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[classmethod]
    fn provides(_cls: &Bound<'_, PyType>) -> String {
        SpecsAnalysis::PROVIDES.to_string()
    }
}

impl PySpecsAnalysis {
    pub fn to_rs(&self) -> SpecsAnalysis {
        self.0.clone()
    }
}

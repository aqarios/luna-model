use lunamodel_transformv2::analysis::CheckModelSpecsAnalysis;
use lunamodel_transpiler::AnalysisPass;
use pyo3::{Bound, pyclass, pymethods, types::PyType};

use crate::PyModelSpecs;

#[pyclass(subclass)]
pub struct PyCheckModelSpecsAnalysis(pub CheckModelSpecsAnalysis);

#[pymethods]
impl PyCheckModelSpecsAnalysis {
    #[new]
    fn new(specs: PyModelSpecs) -> Self {
        Self(CheckModelSpecsAnalysis::new(specs.into()))
    }

    #[classmethod]
    fn provides(_cls: &Bound<'_, PyType>) -> String {
        CheckModelSpecsAnalysis::PROVIDES.to_string()
    }
}

impl PyCheckModelSpecsAnalysis {
    pub fn to_rs(&self) -> CheckModelSpecsAnalysis {
        self.0.clone()
    }
}

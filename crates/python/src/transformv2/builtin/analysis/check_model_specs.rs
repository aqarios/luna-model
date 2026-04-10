use lunamodel_transformv2::analysis::CheckModelSpecsAnalysis;
use lunamodel_transpiler::AnalysisPass;
use pyo3::{Bound, PyResult, pyclass, pymethods, types::PyType};

use crate::{PyModel, PyModelSpecs, transformv2::PyPassContext};

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

    fn name(&self) -> String {
        self.0.name().to_string()
    }

    fn run(&self, model: PyModel, ctx: &PyPassContext) -> PyResult<()> {
        Ok(self.0.run(&model.0.m.read_arc(), &ctx.into())?)
    }

    fn requires(&self) -> Vec<String> {
        self.0.requires().to_vec()
    }
}

impl PyCheckModelSpecsAnalysis {
    pub fn to_rs(&self) -> CheckModelSpecsAnalysis {
        self.0.clone()
    }
}

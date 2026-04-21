use lunamodel_transform::analysis::SpecsAnalysis;
use lunamodel_transpiler::AnalysisPass;
use pyo3::{Bound, PyResult, pyclass, pymethods, types::PyType};

use crate::{PyModel, PyModelSpecs, transform::PyPassContext};

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

    fn name(&self) -> String {
        self.0.name().to_string()
    }

    fn run(&self, model: PyModel, ctx: &PyPassContext) -> PyResult<PyModelSpecs> {
        Ok(self.0.run(&model.0.m.read_arc(), &ctx.into())?.into())
    }

    fn requires(&self) -> Vec<String> {
        self.0.requires().to_vec()
    }

    fn __str__(&self) -> String {
        self.0.display()
    }
}

impl PySpecsAnalysis {
    pub fn to_rs(&self) -> SpecsAnalysis {
        self.0.clone()
    }
}

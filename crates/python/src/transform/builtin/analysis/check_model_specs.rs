//! Python wrapper for explicit model-spec checking analysis.

use lunamodel_python_macros::pyanalysis;
use lunamodel_transform::analysis::CheckModelSpecsAnalysis;
use lunamodel_transpiler::AnalysisPass;
use pyo3::pymethods;

use crate::{PyModel, PyModelSpecs, transform::PyPassContext};

#[pyanalysis]
pub struct PyCheckModelSpecsAnalysis(pub CheckModelSpecsAnalysis);

#[pymethods]
impl PyCheckModelSpecsAnalysis {
    #[new]
    fn new(specs: PyModelSpecs) -> Self {
        Self(CheckModelSpecsAnalysis::new(specs.into()))
    }
}

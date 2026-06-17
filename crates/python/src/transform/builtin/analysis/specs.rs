//! Python wrapper for model-spec inference analysis.

use lunamodel_python_macros::pyanalysis;
use lunamodel_transform::analysis::SpecsAnalysis;
use lunamodel_transpiler::AnalysisPass;
use pyo3::pymethods;

use crate::{PyModel, PyModelSpecs, transform::PyPassContext};

#[pyanalysis(PyModelSpecs)]
#[derive(Default)]
pub struct PySpecsAnalysis(pub SpecsAnalysis);

#[pymethods]
impl PySpecsAnalysis {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}

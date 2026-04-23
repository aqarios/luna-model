use lunamodel_python_macros::pyanalysis;
use lunamodel_transform::analysis::SpecsAnalysis;
use pyo3::pymethods;

use crate::PyModelSpecs;

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

use lunamodel_python_macros::pyanalysis;
use lunamodel_transform::analysis::CheckModelSpecsAnalysis;
use pyo3::pymethods;

use crate::PyModelSpecs;

#[pyanalysis]
pub struct PyCheckModelSpecsAnalysis(pub CheckModelSpecsAnalysis);

#[pymethods]
impl PyCheckModelSpecsAnalysis {
    #[new]
    fn new(specs: PyModelSpecs) -> Self {
        Self(CheckModelSpecsAnalysis::new(specs.into()))
    }
}

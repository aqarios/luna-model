use lunamodel_transpiler::AnalysisManager;
use pyo3::pyclass;

#[pyclass(subclass)]
pub struct PyPassContext {
    manager: AnalysisManager,
}

impl From<AnalysisManager> for PyPassContext {
    fn from(manager: AnalysisManager) -> Self {
        Self { manager }
    }
}

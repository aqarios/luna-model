use lunamodel_transpiler::AnalysisManager;
use pyo3::pyclass;

#[pyclass]
pub struct PyAnalysisManager {
    am: AnalysisManager,
}

impl From<AnalysisManager> for PyAnalysisManager {
    fn from(am: AnalysisManager) -> Self {
        Self { am }
    }
}

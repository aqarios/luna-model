use lunamodel_transpiler::AnalysisManager;
use pyo3::pyclass;

#[pyclass]
pub struct PyAnalysisManager {
    _am: AnalysisManager,
}

impl From<AnalysisManager> for PyAnalysisManager {
    fn from(am: AnalysisManager) -> Self {
        Self { _am: am }
    }
}

use lunamodel_transpiler::{AnalysisKey, AnalysisManager};
use pyo3::{Py, PyAny, PyResult, Python, pyclass, pymethods};

use crate::transformv2::adapter::PyAnalysisPassAdapterResult;

#[pyclass]
pub struct PyPassContext {
    manager: AnalysisManager,
}

impl From<AnalysisManager> for PyPassContext {
    fn from(manager: AnalysisManager) -> Self {
        Self { manager }
    }
}

#[pymethods]
impl PyPassContext {
    fn require_analysis(&self, py: Python, key: String) -> PyResult<Py<PyAny>> {
        let res: &PyAnalysisPassAdapterResult = self.manager.require(&AnalysisKey::new(key))?;
        Ok(res.0.clone_ref(py))
    }
}

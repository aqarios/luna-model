use lunamodel_transpiler::{AnalysisKey, AnalysisManager, PassContext};
use pyo3::{Py, PyAny, PyResult, Python, pyclass, pymethods};

use crate::transformv2::adapter::PyAnalysisPassAdapterResult;

#[pyclass(subclass)]
pub struct PyPassContext {
    manager: AnalysisManager,
}

impl From<AnalysisManager> for PyPassContext {
    fn from(manager: AnalysisManager) -> Self {
        Self { manager }
    }
}

impl<'c> Into<PassContext<'c>> for &'c PyPassContext {
    fn into(self) -> PassContext<'c> {
        PassContext::new(&self.manager)
    }
}

#[pymethods]
impl PyPassContext {
    #[new]
    fn new() -> Self {
        Self {
            manager: AnalysisManager::default(),
        }
    }

    fn require_analysis(&self, py: Python, key: String) -> PyResult<Py<PyAny>> {
        let res: &PyAnalysisPassAdapterResult = self.manager.require(&AnalysisKey::new(key))?;
        Ok(res.0.clone_ref(py))
    }
}

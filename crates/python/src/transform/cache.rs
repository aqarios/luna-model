use lunamodel_transform::{AnalysisCache, AnalysisCacheElement};
use pyo3::{IntoPyObjectExt, Py, PyAny, PyResult, Python, pyclass, pymethods};

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyAnalysisCache {
    pub c: AnalysisCache,
}

impl From<AnalysisCache> for PyAnalysisCache {
    fn from(c: AnalysisCache) -> Self {
        Self { c }
    }
}

#[pymethods]
impl PyAnalysisCache {
    #[new]
    fn new() -> Self {
        Self {
            c: AnalysisCache::new(),
        }
    }

    fn __getitem__(&self, py: Python, key: String) -> PyResult<Option<Py<PyAny>>> {
        if let Some(val) = self.c.get(&key) {
            Ok(Some(match val {
                AnalysisCacheElement::MaxBiasAnalysis(v) => v.clone().into_py_any(py)?,
                AnalysisCacheElement::BinarySpinInfoAnalysis(v) => v.clone().into_py_any(py)?,
                AnalysisCacheElement::IfElseInfoAnalysis(v) => v.clone().into_py_any(py)?,
                AnalysisCacheElement::PyAnalysis(v) => v.into_py_any(py)?,
            }))
        } else {
            Ok(None)
        }
    }
}

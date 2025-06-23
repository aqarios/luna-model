use derive_more::{Deref, DerefMut};
use pyo3::{pyclass, pymethods, IntoPyObjectExt, PyObject, PyResult, Python};

use crate::transformations::{
    analysis_cache::{AnalysisCache, AnalysisCacheElement},
    passes::max_bias::MaxBias,
};

#[pyclass(unsendable, name = "AnalysisCache")]
#[derive(Deref, DerefMut)]
pub struct PyAnalysisCache(pub AnalysisCache);

impl PyAnalysisCache {
    pub fn new(cache: AnalysisCache) -> Self {
        PyAnalysisCache(cache)
    }
}

#[pymethods]
impl PyAnalysisCache {

    fn __getitem__(&self, py: Python, key: String) -> PyResult<Option<PyObject>> {
        self.get_element(py, key)
    }

    #[pyo3(name = "get")]
    pub fn get_element(&self, py: Python, key: String) -> PyResult<Option<PyObject>> {
        if let Some(val) = self.get(&key) {
            Ok(Some(match val {
                AnalysisCacheElement::MaxBiasAnalysis(v) => v.into_py_any(py)?,
                #[cfg(feature = "py")]
                AnalysisCacheElement::PyAnalysis(v) => v.into_py_any(py)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn max_bias(&self) -> Option<MaxBias> {
        if let Some(AnalysisCacheElement::MaxBiasAnalysis(b)) = self.get("max-bias") {
            Some(*b)
        } else {
            None
        }
    }
}

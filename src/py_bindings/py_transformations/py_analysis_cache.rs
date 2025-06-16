use derive_more::{Deref, DerefMut};
use pyo3::{pyclass, pymethods};

use crate::transformations::{analysis_cache::AnalysisCache, passes::max_bias::MaxBias};

#[pyclass]
#[derive(Deref, DerefMut)]
pub struct PyAnalysisCache(AnalysisCache);

impl PyAnalysisCache {
    pub fn new(cache: AnalysisCache) -> Self {
        PyAnalysisCache(cache)
    }
}

#[pymethods]
impl PyAnalysisCache {
    pub fn max_bias(&self) -> Option<MaxBias> {
        self.get::<MaxBias>("max-bias").cloned()
    }
}

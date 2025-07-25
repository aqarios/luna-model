use std::fmt::Debug;

use pyo3::prelude::*;

use crate::{
    py_bindings::AnyPass,
    transformations::{analysis_cache::PyAnalysisCache, base_passes::Pass},
};

use super::{py_meta_analysis_adapter::PyMetaAnalysisPassAdapter, py_pass_base::PyPass};

#[pyclass(unsendable, subclass, name = "MetaAnalysisPass")]
#[derive(Debug)]
pub struct PyMetaAnalysisPass {}

#[pymethods]
impl PyMetaAnalysisPass {
    #[new]
    #[pyo3(signature=(*args, **kwargs))]
    #[allow(unused_variables)]
    fn py_new(args: &Bound<'_, PyAny>, kwargs: Option<&Bound<'_, PyAny>>) -> Self {
        Self {}
    }

    #[getter]
    #[pyo3(name = "name")]
    fn get_name(&self) -> PyResult<String> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "'name' property is not implemented.",
        ))
    }

    #[getter]
    #[pyo3(name = "requires")]
    fn get_requires(&self) -> Vec<String> {
        Vec::new()
    }

    #[pyo3(name = "run")]
    #[allow(unused_variables)]
    fn py_run(&self, pipeline: Vec<AnyPass>, cache: &PyAnalysisCache) -> PyResult<Py<PyAny>> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "'run' method is not implemented.",
        ))
    }
}

impl PyPass for Py<PyMetaAnalysisPass> {
    fn as_pass(self) -> PyResult<Pass> {
        Ok(Pass::MetaAnalysis(Box::new(
            PyMetaAnalysisPassAdapter::new(self)?,
        )))
    }
}

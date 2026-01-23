use crate::py::passes::PyPass;
use pyo3::{Bound, Py, PyAny, PyResult, pyclass, pymethods};

use super::adapter::PyMetaAnalysisPassAdapter;
use crate::py::AnyPass;
use crate::{base::Pass, cache::PyAnalysisCache};

#[pyclass(subclass, name = "MetaAnalysisPass")]
#[derive(Debug)]
pub struct PyMetaAnalysisPass {}

#[pymethods]
impl PyMetaAnalysisPass {
    #[new]
    #[pyo3(signature=(*args, **kwargs))]
    fn py_new(args: &Bound<'_, PyAny>, kwargs: Option<&Bound<'_, PyAny>>) -> Self {
        _ = args;
        _ = kwargs;
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
    fn py_run(&self, pipeline: Vec<AnyPass>, cache: &PyAnalysisCache) -> PyResult<Py<PyAny>> {
        _ = pipeline;
        _ = cache;
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

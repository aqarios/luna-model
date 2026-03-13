use lunamodel_unwind::*;
use pyo3::{Bound, Py, PyAny, PyResult, pyclass, pymethods};

use crate::{model::PyModel, transform::cache::PyAnalysisCache};

#[pyclass(subclass)]
#[derive(Debug)]
pub struct PyAnalysisPass {}

#[unwindable]
#[pymethods]
impl PyAnalysisPass {
    #[new]
    #[pyo3(signature=(*args, **kwargs))]
    fn py_new(args: &Bound<'_, PyAny>, kwargs: Option<&Bound<'_, PyAny>>) -> Self {
        _ = args;
        _ = kwargs;
        Self {}
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "'name' property is not implemented.",
        ))
    }

    #[getter]
    fn requires(&self) -> Vec<String> {
        Vec::new()
    }

    fn run(&self, model: PyModel, cache: &PyAnalysisCache) -> PyResult<Py<PyAny>> {
        _ = model;
        _ = cache;
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "'run' method is not implemented.",
        ))
    }
}

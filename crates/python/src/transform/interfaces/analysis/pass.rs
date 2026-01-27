use pyo3::{Bound, Py, PyAny, PyResult, pyclass, pymethods};

use crate::{model::PyModel, transform::cache::PyAnalysisCache};

#[pyclass(subclass)]
#[derive(Debug)]
pub struct PyAnalysisPass {}

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
    fn py_run(&self, model: PyModel, cache: &PyAnalysisCache) -> PyResult<Py<PyAny>> {
        _ = model;
        _ = cache;
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "'run' method is not implemented.",
        ))
    }
}

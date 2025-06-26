use std::fmt::Debug;

use pyo3::{prelude::*, types::PyDict};

use crate::{
    py_bindings::{py_model::PyModel, py_transformations::py_analysis_cache::PyAnalysisCache},
    transformations::base_passes::Pass,
};

use super::{py_analysis_pass_adapter::PyAnalysisPassAdapter, py_pass_base::PyPass};

#[pyclass(unsendable, subclass, name = "AnalysisPass")]
#[derive(Clone, Debug)]
pub struct PyAnalysisPass {}

#[pymethods]
impl PyAnalysisPass {
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
    fn py_run(&self, model: PyModel, cache: &PyAnalysisCache) -> PyResult<Py<PyDict>> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "'run' method is not implemented.",
        ))
    }
}

impl PyPass for Py<PyAnalysisPass> {
    fn as_pass(self) -> PyResult<Pass> {
        Ok(Pass::Analysis(Box::new(PyAnalysisPassAdapter::new(self)?)))
    }
}

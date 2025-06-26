use std::fmt::Debug;

use pyo3::prelude::*;

use super::{py_pass_base::PyPass, py_transformation_pass_adapter::PyTransformationPassAdapter};
use crate::{
    py_bindings::{py_model::PyModel, py_sol::PySolution, py_transformations::py_analysis_cache::PyAnalysisCache},
    transformations::base_passes::{Pass, TransformationType},
};

#[pyclass(unsendable, subclass, name = "TransformationPass")]
#[derive(Clone, Debug)]
pub struct PyTransformationPass {}

#[pymethods]
impl PyTransformationPass {
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

    #[getter]
    #[pyo3(name = "invalidates")]
    fn get_invalidates(&self) -> Vec<String> {
        Vec::new()
    }

    #[pyo3(name = "run")]
    #[allow(unused_variables)]
    fn py_run(
        &self,
        model: PyModel,
        cache: &PyAnalysisCache,
    ) -> PyResult<(PyModel, TransformationType)> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "'run' method is not implemented.",
        ))
    }

    #[pyo3(name = "backwards")]
    #[allow(unused_variables)]
    fn py_backwards(&self, solution: &PySolution, cache: &PyAnalysisCache) -> PyResult<PySolution> {
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "'backwards' method is not implemented.",
        ))
    }
}

impl PyPass for Py<PyTransformationPass> {
    fn as_pass(self) -> PyResult<Pass> {
        Ok(Pass::Transformation(Box::new(
            PyTransformationPassAdapter::new(self)?,
        )))
    }
}

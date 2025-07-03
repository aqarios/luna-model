use std::{fmt::Debug, rc::Rc};

use pyo3::prelude::*;

use super::{py_pass_base::PyPass, py_transformation_pass_adapter::PyTransformationPassAdapter};
use crate::{
    py_bindings::{py_model::PyModel, py_sol::PySolution},
    transformations::{
        analysis_cache::{AnalysisCacheElement, PyAnalysisCache},
        base_passes::{ActionType, Pass, TransformationOutcome},
    },
};

#[pyclass(unsendable, name = "TransformationOutcome", get_all, set_all)]
#[derive(FromPyObject)]
pub struct PyTransformationOutcome {
    pub model: PyModel,
    pub action: ActionType,
    pub analysis: Option<Py<PyAny>>,
}

#[pymethods]
impl PyTransformationOutcome {
    #[new]
    #[pyo3(signature=(model, action, analysis=None))]
    pub fn py_new(model: PyModel, action: ActionType, analysis: Option<Py<PyAny>>) -> Self {
        PyTransformationOutcome {
            model,
            action,
            analysis,
        }
    }
}

impl TryInto<TransformationOutcome> for PyTransformationOutcome {
    type Error = String;

    fn try_into(self) -> Result<TransformationOutcome, String> {
        let model = Rc::into_inner(self.model.concrete_model)
            .ok_or("Model reference leaked out of transformation scope.".to_string())?
            .into_inner();
        Ok(TransformationOutcome {
            model,
            analysis: self.analysis.map(|x| AnalysisCacheElement::PyAnalysis(x)),
            action: self.action.clone(),
        })
    }
}

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
    fn py_run(&self, model: PyModel, cache: &PyAnalysisCache) -> PyResult<PyTransformationOutcome> {
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

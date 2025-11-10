use super::{py_pass_base::PyPass, py_transformation_pass_adapter::PyTransformationPassAdapter};
use crate::{
    py_bindings::{py_model::PyModel, py_sol::PySolution, unwind},
    transformations::{
        analysis_cache::{AnalysisCacheElement, PyAnalysisCache},
        base_passes::{ActionType, Pass, TransformationOutcome},
    },
    utils::ShareMut,
};
use pyo3::prelude::*;
use pyo3::types::PyNone;
use std::fmt::Debug;
use unwind_macros::unwindable;

#[pyclass(name = "TransformationOutcome", get_all, set_all)]
#[derive(FromPyObject)]
pub struct StructuredPyTransformationOutcome {
    pub model: PyModel,
    pub action: ActionType,
    pub analysis: Option<Py<PyAny>>,
}

#[derive(FromPyObject)]
pub enum PyTransformationOutcome {
    Structured(StructuredPyTransformationOutcome),
    Tuple2(PyModel, ActionType),
    Tuple3(PyModel, ActionType, Option<Py<PyAny>>),
}

#[pymethods]
impl StructuredPyTransformationOutcome {
    #[new]
    #[pyo3(signature=(model, action, analysis=None))]
    pub fn py_new(model: PyModel, action: ActionType, analysis: Option<Py<PyAny>>) -> Self {
        StructuredPyTransformationOutcome {
            model,
            action,
            analysis,
        }
    }

    #[staticmethod]
    pub fn nothing(model: PyModel) -> Self {
        StructuredPyTransformationOutcome {
            model,
            action: ActionType::DidNothing,
            analysis: None,
        }
    }
}

impl TryInto<TransformationOutcome> for PyTransformationOutcome {
    type Error = String;

    fn try_into(self) -> Result<TransformationOutcome, String> {
        let (pymodel, action, analysis) = match self {
            PyTransformationOutcome::Structured(x) => (x.model, x.action, x.analysis),
            PyTransformationOutcome::Tuple2(a, b) => (a, b, None),
            PyTransformationOutcome::Tuple3(a, b, c) => (a, b, c),
        };
        let model = ShareMut::into_inner(pymodel.concrete_model)
            .ok_or("Model reference leaked out of transformation scope.".to_string())?;
        Ok(TransformationOutcome {
            model,
            analysis: analysis.map(|x| AnalysisCacheElement::PyAnalysis(x)),
            action,
        })
    }
}

#[pyclass(subclass, name = "TransformationPass")]
#[derive(Clone, Debug)]
pub struct PyTransformationPass {}

#[unwindable]
#[pymethods]
impl PyTransformationPass {
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

    #[getter]
    #[pyo3(name = "invalidates")]
    fn get_invalidates(&self) -> Vec<String> {
        Vec::new()
    }

    #[pyo3(name = "run")]
    fn py_run(&self, model: PyModel, cache: &PyAnalysisCache) -> PyResult<Py<PyNone>> {
        _ = model;
        _ = cache;
        Err(pyo3::exceptions::PyNotImplementedError::new_err(
            "'run' method is not implemented.",
        ))
    }

    #[pyo3(name = "backwards")]
    fn py_backwards(&self, solution: &PySolution, cache: &PyAnalysisCache) -> PyResult<PySolution> {
        _ = solution;
        _ = cache;
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

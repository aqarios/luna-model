use std::sync::Arc;

use lunamodel_unwind::*;
use lunamodel_transform::{ActionType, AnalysisCacheElement, TransformationOutcome};
use pyo3::{FromPyObject, IntoPyObject, Py, PyAny, PyErr, PyResult, Python, pyclass, pymethods};

use crate::model::PyModel;

#[pyclass(name = "PyTransformationOutcome", get_all, set_all)]
#[derive(FromPyObject)]
pub struct PyStructuredTransformationOutcome {
    pub model: PyModel,
    pub action: ActionType,
    pub analysis: Option<Py<PyAny>>,
}

#[unwindable]
#[pymethods]
impl PyStructuredTransformationOutcome {
    #[new]
    #[pyo3(signature=(model, action, analysis=None))]
    pub fn new(model: PyModel, action: ActionType, analysis: Option<Py<PyAny>>) -> Self {
        PyStructuredTransformationOutcome {
            model,
            action,
            analysis,
        }
    }

    #[staticmethod]
    pub fn nothing(model: PyModel) -> Self {
        PyStructuredTransformationOutcome {
            model,
            action: ActionType::DidNothing,
            analysis: None,
        }
    }
}

#[derive(FromPyObject, IntoPyObject)]
pub enum PyTransformationOutcome {
    Structured(PyStructuredTransformationOutcome),
    Tuple2(PyModel, ActionType),
    Tuple3(PyModel, ActionType, Option<Py<PyAny>>),
}

impl TryInto<TransformationOutcome> for PyTransformationOutcome {
    type Error = String;

    fn try_into(self) -> Result<TransformationOutcome, Self::Error> {
        let (pymodel, action, analysis) = match self {
            PyTransformationOutcome::Structured(x) => (x.model, x.action, x.analysis),
            PyTransformationOutcome::Tuple2(a, b) => (a, b, None),
            PyTransformationOutcome::Tuple3(a, b, c) => (a, b, c),
        };
        let model = Arc::into_inner(pymodel.m)
            .ok_or("Model reference leaked out of transformation scope.".to_string())?
            .into_inner();

        Ok(TransformationOutcome {
            model,
            analysis: analysis.map(|x| AnalysisCacheElement::PyAnalysis(x)),
            action,
        })
    }
}

impl PyTransformationOutcome {
    fn from_actual(outcome: TransformationOutcome, py: Python) -> PyResult<Self> {
        let analysis = match outcome.analysis {
            Some(element) => Some(element.into_py_any(py)?),
            None => None,
        };
        Ok(PyTransformationOutcome::Structured(
            PyStructuredTransformationOutcome {
                model: outcome.model.into(),
                action: outcome.action,
                analysis,
            },
        ))
    }
}

impl<'py> TryFrom<(TransformationOutcome, Python<'py>)> for PyTransformationOutcome {
    type Error = PyErr;
    fn try_from(value: (TransformationOutcome, Python)) -> Result<Self, Self::Error> {
        let (outcome, py) = value;
        Self::from_actual(outcome, py)
    }
}

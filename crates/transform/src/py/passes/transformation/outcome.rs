use std::sync::Arc;

use lunamodel_python::PyModel;
use pyo3::{FromPyObject, Py, PyAny, pyclass, pymethods};

use crate::{
    base::{ActionType, TransformationOutcome},
    cache::AnalysisCacheElement,
};

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

impl TryInto<TransformationOutcome> for PyTransformationOutcome {
    type Error = String;

    fn try_into(self) -> Result<TransformationOutcome, String> {
        let (pymodel, action, analysis) = match self {
            PyTransformationOutcome::Structured(x) => (x.model, x.action, x.analysis),
            PyTransformationOutcome::Tuple2(a, b) => (a, b, None),
            PyTransformationOutcome::Tuple3(a, b, c) => (a, b, c),
        };
        // let model = pymodel.m.
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

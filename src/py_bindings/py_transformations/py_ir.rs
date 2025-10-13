use crate::py_bindings::unwind;
use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;
use unwind_macros::unwindable;

use crate::{
    py_bindings::{py_model::PyModel, py_timing::PyTiming},
    transformations::{
        analysis_cache::PyAnalysisCache,
        base_passes::ActionType,
        intermediate_representation::{IntermediateRepresentation, LogElement},
    },
};

#[cfg_attr(
    not(feature = "lq"),
    pyclass(
        get_all,
        name = "LogElement",
        module = "aqmodels._core.transformations"
    )
)]
#[cfg_attr(
    feature = "lq",
    pyclass(
        get_all,
        name = "LogElement",
        module = "luna_quantum._core.transformations"
    )
)]
#[derive(Clone)]
pub struct PyLogElement {
    pass_name: String,
    timing: PyTiming,
    kind: ActionType,
    components: Option<Vec<PyLogElement>>,
}
impl PyLogElement {
    fn new(elem: &LogElement) -> Self {
        Self {
            pass_name: elem.pass.clone(),
            timing: PyTiming(elem.timing),
            kind: elem.kind.clone(),
            components: elem
                .components
                .as_ref()
                .map(|x| x.iter().map(|e| PyLogElement::new(e)).collect()),
        }
    }
}

#[pymethods]
impl PyLogElement {
    fn __repr__(&self) -> String {
        format!(
            "Log(\"{}\", action={:?}, components={:?})",
            self.pass_name,
            self.kind,
            self.components.as_ref().map(|x| x.len())
        )
    }
}

#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "IR", module = "aqmodels._core.transformations")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "IR", module = "luna_quantum._core.transformations")
)]
#[derive(Deref, DerefMut)]
pub struct PyIR(pub IntermediateRepresentation);

#[unwindable]
#[pymethods]
impl PyIR {
    #[getter]
    #[pyo3(name = "model")]
    fn py_model(&self) -> PyModel {
        PyModel::new(self.model.clone())
    }

    #[getter]
    #[pyo3(name = "cache")]
    fn py_cache(&self, py: Python) -> PyAnalysisCache {
        PyAnalysisCache::new(self.cache.clone_py(py))
    }

    #[getter]
    #[pyo3(name = "execution_log")]
    fn py_execution_log(&self) -> Vec<PyLogElement> {
        self.execution_log
            .iter()
            .map(|elem| PyLogElement::new(&elem))
            .collect()
    }
}

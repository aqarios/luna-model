use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

use crate::{
    py_bindings::{py_model::PyModel, py_timing::PyTiming},
    transformations::{
        analysis_cache::PyAnalysisCache,
        base_passes::TransformationType,
        intermediate_representation::{IntermediateRepresentation, LogElement},
    },
};

#[cfg_attr(
    not(feature = "lq"),
    pyclass(
        unsendable,
        get_all,
        name = "LogElement",
        module = "aqmodels.transformations"
    )
)]
#[cfg_attr(
    feature = "lq",
    pyclass(
        unsendable,
        get_all,
        name = "LogElement",
        module = "luna_quantum.transformations"
    )
)]
pub struct PyLogElement {
    pass_name: String,
    timing: PyTiming,
    kind: Option<TransformationType>,
}
impl PyLogElement {
    fn new(elem: &LogElement) -> Self {
        Self {
            pass_name: elem.pass.clone(),
            timing: PyTiming(elem.timing),
            kind: elem.kind.clone(),
        }
    }
}

#[cfg_attr(not(feature = "lq"), pyclass(unsendable, name = "IR", module = "aqmodels.transformations"))]
#[cfg_attr(feature = "lq", pyclass(unsendable, name = "IR", module = "luna_quantum.transformations"))]
#[derive(Deref, DerefMut)]
pub struct PyIR(pub IntermediateRepresentation);

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

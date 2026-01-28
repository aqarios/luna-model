use lunamodel_transform::IR;
use pyo3::{pyclass, pymethods};

use crate::{
    PyModel,
    transform::{PyAnalysisCache, PyLogElement},
};

#[pyclass]
#[repr(C)]
pub struct PyIR {
    pub ir: IR,
}

impl From<IR> for PyIR {
    fn from(ir: IR) -> Self {
        Self { ir }
    }
}

#[pymethods]
impl PyIR {
    #[getter]
    fn model(&self) -> PyModel {
        self.ir.model.clone().into()
    }

    #[getter]
    fn cache(&self) -> PyAnalysisCache {
        self.ir.cache.clone().into()
    }

    #[getter]
    fn execution_log(&self) -> Vec<PyLogElement> {
        self.ir
            .execution_log
            .iter()
            .map(|l| l.clone().into())
            .collect()
    }
}

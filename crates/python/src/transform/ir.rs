use lunamodel_transform::IR;
use pyo3::{pyclass, pymethods};

use crate::{PyModel, transform::PyAnalysisCache};

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
}

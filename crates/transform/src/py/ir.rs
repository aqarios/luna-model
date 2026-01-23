use derive_more::{Deref, DerefMut};
use lunamodel_python::PyModel;
use lunamodel_unwind::*;
use pyo3::prelude::{Python, pyclass, pymethods};

use super::log::PyLogElement;
use crate::{cache::PyAnalysisCache, ir::IR};

#[pyclass(name = "IR")]
#[derive(Deref, DerefMut)]
pub struct PyIR(pub IR);

#[unwindable]
#[pymethods]
impl PyIR {
    #[getter]
    fn model(&self) -> PyModel {
        self.model.clone().into()
    }

    #[getter]
    fn cache(&self, py: Python) -> PyAnalysisCache {
        PyAnalysisCache::new(self.cache.clone_py(py))
    }

    #[getter]
    fn execution_log(&self) -> Vec<PyLogElement> {
        self.execution_log
            .iter()
            .map(|elem| PyLogElement::new(&elem))
            .collect()
    }
}

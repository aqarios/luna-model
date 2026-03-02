use lunamodel_transform::{
    AnalysisPass, BasePass, passes::analysis::MinValueForConstraintAnalysis,
};
use lunamodel_unwind::*;
use pyo3::{Py, PyAny, PyResult, Python, pyclass, pymethods};

use crate::{
    PyModel,
    transform::{PyAnalysisCache, cache::AnalysisCacheElementPyMethods},
};

#[pyclass]
pub struct PyMinValueForConstraintsAnalysis {
    p: MinValueForConstraintAnalysis,
}

#[unwindable]
#[pymethods]
impl PyMinValueForConstraintsAnalysis {
    #[new]
    fn new() -> Self {
        Self {
            p: MinValueForConstraintAnalysis::new(),
        }
    }

    #[getter]
    fn name(&self) -> String {
        self.p.name()
    }

    #[getter]
    fn requires(&self) -> Vec<String> {
        self.p.requires()
    }

    fn run(
        &self,
        py: Python,
        model: &PyModel,
        cache: &PyAnalysisCache,
    ) -> PyResult<Option<Py<PyAny>>> {
        let res = self.p.run(&model.m.read_arc(), &cache.c)?;
        match res {
            Some(e) => Ok(Some(e.into_py_any(py)?)),
            None => Ok(None),
        }
    }
}

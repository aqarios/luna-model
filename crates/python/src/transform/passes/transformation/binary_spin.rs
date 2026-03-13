use lunamodel_transform::{BasePass, TransformationPass, passes::BinarySpinPass};
use lunamodel_unwind::*;
use pyo3::{PyResult, Python, pyclass, pymethods};

use crate::{
    PyModel, PySolution,
    transform::{PyAnalysisCache, interfaces::PyTransformationOutcome},
    types::PyVtype,
};

#[derive(Debug, Clone)]
#[pyclass]
pub struct PyBinarySpinPass {
    pub p: BinarySpinPass,
}

#[unwindable]
#[pymethods]
impl PyBinarySpinPass {
    #[new]
    fn new(vtype: PyVtype, prefix: Option<String>) -> Self {
        Self {
            p: BinarySpinPass::new(vtype.into(), prefix),
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

    #[getter]
    fn invalidates(&self) -> Vec<String> {
        self.p.invalidates()
    }

    fn run(
        &self,
        py: Python,
        model: PyModel,
        cache: &PyAnalysisCache,
    ) -> PyResult<PyTransformationOutcome> {
        let pyto: PyTransformationOutcome =
            (self.p.run(model.m.read_arc().deep_clone(), &cache.c)?, py).try_into()?;
        Ok(pyto)
    }

    fn backwards(&self, solution: &PySolution, cache: &PyAnalysisCache) -> PyResult<PySolution> {
        Ok(self
            .p
            .backwards(solution.s.read_arc().clone(), &cache.c)?
            .into())
    }
}

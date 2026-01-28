use lunamodel_transform::{BasePass, TransformationPass, passes::ChangeSensePass};
use lunamodel_unwind::*;
use lunamodel_types::Sense;
use pyo3::{PyResult, Python, pyclass, pymethods};

use crate::{
    model::PyModel,
    sol::PySolution,
    transform::{cache::PyAnalysisCache, interfaces::PyTransformationOutcome},
};

#[derive(Debug)]
#[pyclass]
pub struct PyChangeSensePass {
    pub p: ChangeSensePass,
}

#[unwindable]
#[pymethods]
impl PyChangeSensePass {
    #[new]
    fn new(sense: Sense) -> Self {
        Self {
            p: ChangeSensePass::new(sense),
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
            .backwards(solution.s.read_arc().clone(), &cache.c)
            .into())
    }
}

use std::{collections::HashSet, fmt::Debug};

use lunamodel_core::{Model, Solution};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transform::{
    AnalysisCache, BasePass, ExecutionLog, IR, Pass, PassManager,
    passes::special::{AbstractPipeline, PipelineResult},
};
use pyo3::{Py, PyErr, PyResult, Python};

use crate::transform::pipeline::PyPipeline;

pub struct PyPipelineAdapter {
    pub(crate) inner: Py<PyPipeline>,
}

impl PyPipelineAdapter {
    pub fn new(inner: Py<PyPipeline>) -> PyResult<Self> {
        let slf = Self { inner };
        Ok(slf)
    }
}

impl BasePass for PyPipelineAdapter {
    fn name(&self) -> String {
        Python::attach(|py| self.inner.extract::<PyPipeline>(py).unwrap().p.name())
    }

    fn requires(&self) -> Vec<String> {
        Python::attach(|py| self.inner.extract::<PyPipeline>(py).unwrap().p.requires())
    }
}

impl AbstractPipeline for PyPipelineAdapter {
    fn run(&self, model: Model, cache: &AnalysisCache, executor: &PassManager) -> PipelineResult {
        Python::attach(|py| {
            self.inner
                .extract::<PyPipeline>(py)
                .unwrap()
                .p
                .run(model, cache, executor)
        })
    }

    fn backwards(
        &self,
        solution: Solution,
        ir: &IR,
        log: &ExecutionLog,
    ) -> LunaModelResult<Solution> {
        Python::attach(|py| {
            self.inner
                .extract::<PyPipeline>(py)
                .map_err(|e: pyo3::pyclass::PyClassGuardError<'_, '_>| {
                    let mapped = LunaModelError::Internal(e.to_string().into());
                    let pye: PyErr = e.into();
                    LunaModelError::WithCause(Box::new(mapped), pye.into())
                })?
                .p
                .backwards(solution, ir, log)
        })
    }

    fn clear(&mut self) {
        Python::attach(|py| self.inner.extract::<PyPipeline>(py).unwrap().p.clear())
    }

    fn add(&mut self, pass: Pass) {
        Python::attach(|py| self.inner.extract::<PyPipeline>(py).unwrap().p.add(pass))
    }

    fn satisfies(&self) -> HashSet<String> {
        Python::attach(|py| self.inner.extract::<PyPipeline>(py).unwrap().p.satisfies())
    }

    fn content_string(&self) -> String {
        Python::attach(|py| {
            self.inner
                .extract::<PyPipeline>(py)
                .unwrap()
                .p
                .content_string()
        })
    }

    fn len(&self) -> usize {
        Python::attach(|py| self.inner.extract::<PyPipeline>(py).unwrap().p.len())
    }

    fn passes(&self) -> Vec<Pass> {
        let x = Python::attach(|py| self.inner.extract::<PyPipeline>(py).unwrap());
        x.p.passes().clone()
    }

    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
}

impl Debug for PyPipelineAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Clone for PyPipelineAdapter {
    fn clone(&self) -> Self {
        Python::attach(|py| PyPipelineAdapter {
            inner: self.inner.clone_ref(py),
        })
    }
}

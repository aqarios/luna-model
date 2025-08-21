use std::{collections::HashSet, fmt::Debug};

use pyo3::prelude::*;

use crate::{
    core::{Model, Solution}, py_bindings::{AnyPass, IntoAnyPass}, transformations::{
        analysis_cache::AnalysisCache,
        base_passes::{self, BasePass, Pass},
        intermediate_representation::IntermediateRepresentation,
        pass_manager::PassManager,
        passes::pipeline::{AbstractPipeline, PipelineResult},
    }
};

use super::PyPipeline;

pub struct PyPipelineAdapter {
    inner: Py<PyPipeline>,
}

impl PyPipelineAdapter {
    pub fn new(inner: Py<PyPipeline>) -> PyResult<Self> {
        let slf = Self { inner };
        Ok(slf)
    }
}

impl BasePass for PyPipelineAdapter {
    fn name(&self) -> String {
        Python::with_gil(|py| self.inner.extract::<PyPipeline>(py).unwrap().0.name())
    }

    fn requires(&self) -> Vec<String> {
        Python::with_gil(|py| self.inner.extract::<PyPipeline>(py).unwrap().0.requires())
    }
}

impl AbstractPipeline for PyPipelineAdapter {
    fn run(&self, model: Model, cache: &AnalysisCache, executor: &PassManager) -> PipelineResult {
        let res = Python::with_gil(|py| {
            self.inner
                .extract::<PyPipeline>(py)
                .unwrap()
                .0
                .run(model, cache, executor)
        });
        dbg!(&res);
        return res
    }

    fn backwards(&self, solution: Solution, ir: &IntermediateRepresentation) -> Solution {
        Python::with_gil(|py| {
            self.inner
                .extract::<PyPipeline>(py)
                .unwrap()
                .0
                .backwards(solution, ir)
        })
    }

    fn clear(&mut self) {
        Python::with_gil(|py| self.inner.extract::<PyPipeline>(py).unwrap().0.clear())
    }

    fn add(&mut self, pass: base_passes::Pass) {
        Python::with_gil(|py| self.inner.extract::<PyPipeline>(py).unwrap().0.add(pass))
    }

    fn satisfies(&self) -> HashSet<String> {
        Python::with_gil(|py| self.inner.extract::<PyPipeline>(py).unwrap().0.satisfies())
    }

    fn content_string(&self) -> String {
        Python::with_gil(|py| {
            self.inner
                .extract::<PyPipeline>(py)
                .unwrap()
                .0
                .content_string()
        })
    }

    fn len(&self) -> usize {
        Python::with_gil(|py| self.inner.extract::<PyPipeline>(py).unwrap().0.len())
    }

    fn passes(&self) -> Vec<Pass> {
        let x = Python::with_gil(|py| self.inner.extract::<PyPipeline>(py).unwrap());
        x.0.passes().clone()
    }
}

impl Debug for PyPipelineAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Clone for PyPipelineAdapter {
    fn clone(&self) -> Self {
        Python::with_gil(|py| PyPipelineAdapter {
            inner: self.inner.clone_ref(py),
        })
    }
}

impl IntoAnyPass for PyPipelineAdapter {
    fn as_anypass(&self) -> AnyPass {
        Python::with_gil(|py| AnyPass::PyPipeline(self.inner.clone_ref(py)))
    }
}


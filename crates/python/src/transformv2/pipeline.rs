use lunamodel_transpiler::{Pipeline, PipelineStep};
use pyo3::{PyResult, Python, pyclass, pymethods};

use crate::transformv2::pass::PyPass;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct PyPipeline(Pipeline);

impl PyPipeline {
    pub fn steps(self) -> Vec<PipelineStep> {
        self.0.steps
    }
}

#[pymethods]
impl PyPipeline {
    #[new]
    fn new(py: Python, name: String, steps: Vec<PyPass>) -> PyResult<Self> {
        Ok(Self(Pipeline::new(
            name,
            steps
                .into_iter()
                .map(|p| p.to_step(py))
                .collect::<PyResult<_>>()?,
        )))
    }

    fn requires(&self) -> Vec<String> {
        self.0.requires().collect()
    }

    fn invalidates(&self) -> Vec<String> {
        self.0.invalidates().collect()
    }

    fn provides(&self) -> Vec<String> {
        self.0.provides().collect()
    }
}

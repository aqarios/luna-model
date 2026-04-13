use lunamodel_transpiler::{Pipeline, PipelineStep};
use pyo3::pyclass;

#[pyclass]
#[derive(Clone)]
pub struct PyPipeline(Pipeline);

impl PyPipeline {
    pub fn steps(self) -> Vec<PipelineStep> {
        self.0.steps
    }
}

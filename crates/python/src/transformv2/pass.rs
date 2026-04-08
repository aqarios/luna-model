use lunamodel_transpiler::PipelineStep;
use pyo3::pyclass;

#[pyclass]
pub struct PyPass {
    inner: PipelineStep,
}

impl PyPass {
    pub fn to_step(&self) -> PipelineStep {
        self.inner.clone()
    }
}

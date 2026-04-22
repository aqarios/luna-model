use derive_more::Deref;
use lunamodel_transform::pipelines::ToUnconstrainedBinaryPipeline;
use pyo3::{pyclass, pymethods};

#[pyclass(subclass)]
#[derive(Deref)]
pub struct PyToUnconstrainedBinaryPipeline(pub ToUnconstrainedBinaryPipeline);

#[pymethods]
impl PyToUnconstrainedBinaryPipeline {
    #[new]
    fn new(penalty_scaling: f64) -> Self {
        Self(ToUnconstrainedBinaryPipeline::new(penalty_scaling))
    }

    fn __str__(&self) -> String {
        self.display()
    }
}

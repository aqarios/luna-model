//! Python wrapper for the built-in binary-minimization pipeline.

use derive_more::Deref;
use lunamodel_transform::pipelines::ToBinaryMinimizationPipeline;
use pyo3::{pyclass, pymethods};

#[pyclass(subclass)]
#[derive(Deref)]
pub struct PyToBinaryMinimizationPipeline(pub ToBinaryMinimizationPipeline);

#[pymethods]
impl PyToBinaryMinimizationPipeline {
    #[new]
    fn new() -> Self {
        Self(ToBinaryMinimizationPipeline::new())
    }

    fn __str__(&self) -> String {
        self.display()
    }
}

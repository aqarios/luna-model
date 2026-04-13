use lunamodel_transform::pipelines::ToBinaryMinimizationPipeline;
use lunamodel_unwind::*;
use pyo3::{pyclass, pymethods};

use crate::transform::PyPipeline;

#[derive(Debug, Clone)]
#[pyclass]
pub struct PyToBinaryMinimizationPipeline;

#[unwindable]
#[pymethods]
impl PyToBinaryMinimizationPipeline {
    #[staticmethod]
    fn create() -> PyPipeline {
        PyPipeline {
            p: ToBinaryMinimizationPipeline::new(),
        }
    }
}

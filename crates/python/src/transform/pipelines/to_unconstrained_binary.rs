use lunamodel_transform::pipelines::ToUnconstrainedBinaryPipeline;
use lunamodel_unwind::*;
use pyo3::{pyclass, pymethods};

use crate::transform::PyPipeline;

#[derive(Debug, Clone)]
#[pyclass]
pub struct PyToUnconstrainedBinaryPipeline;

#[unwindable]
#[pymethods]
impl PyToUnconstrainedBinaryPipeline {
    #[staticmethod]
    fn create(penalty_factor: f64) -> PyPipeline {
        PyPipeline {
            p: ToUnconstrainedBinaryPipeline::new(penalty_factor),
        }
    }
}

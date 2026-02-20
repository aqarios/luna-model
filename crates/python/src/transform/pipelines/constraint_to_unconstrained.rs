use lunamodel_transform::pipelines::NaiveConstrainedToUnconstrainedPipeline;
use lunamodel_unwind::*;
use pyo3::{pyclass, pymethods};

use crate::transform::PyPipeline;

#[derive(Debug, Clone)]
#[pyclass]
pub struct PyConstrainedToUnconstrainedPipeline;

#[unwindable]
#[pymethods]
impl PyConstrainedToUnconstrainedPipeline {
    #[staticmethod]
    fn create(penalty_factor: f64) -> PyPipeline {
        PyPipeline {
            p: NaiveConstrainedToUnconstrainedPipeline::new(penalty_factor),
        }
    }
}

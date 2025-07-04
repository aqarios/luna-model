use pyo3::prelude::*;

use crate::{
    py_bindings::AnyPass,
    transformations::{base_passes::Pass, pipeline::Pipeline},
};

#[pyclass(unsendable, name = "Pipeline")]
#[derive(Debug, Clone)]
pub struct PyPipeline(pub Pipeline);

#[pymethods]
impl PyPipeline {
    #[new]
    #[pyo3(signature = (passes, name=None))]
    fn py_new(passes: Vec<AnyPass>, name: Option<String>) -> PyResult<Self> {
        let mapped = passes
            .into_iter()
            .map(|y| y.as_pass())
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self(Pipeline::new(mapped, name)))
    }
}

impl PyPipeline {
    pub fn as_pass(self) -> PyResult<Pass> {
        Ok(Pass::Pipeline(self.0))
    }
}

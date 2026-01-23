use std::collections::HashSet;

use pyo3::{Py, PyResult, Python, pyclass, pymethods};

use super::PyPipelineAdapter;
use crate::base::BasePass;
use crate::passes::special::AbstractPipeline;
use crate::py::{AnyPass, IntoAnyPass, passes::PyPass};
use crate::{base::Pass, passes::special::Pipeline};

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

    #[getter]
    fn name(&self) -> String {
        return self.0.name();
    }

    #[getter]
    fn requires(&self) -> Vec<String> {
        return self.0.requires();
    }

    #[getter]
    fn satisfies(&self) -> HashSet<String> {
        return self.0.satisfies();
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn add(&mut self, pass: AnyPass) -> PyResult<()> {
        self.0.add(pass.as_pass()?);
        Ok(())
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    #[getter]
    fn passes(&self) -> Vec<AnyPass> {
        self.0.passes().iter().map(|e| e.as_anypass()).collect()
    }

    fn __repr__(&self) -> String {
        format!("{}", self.0)
    }
}

impl PyPass for Py<PyPipeline> {
    fn as_pass(self) -> PyResult<Pass> {
        Ok(Pass::Pipeline(Box::new(PyPipelineAdapter::new(self)?)))
    }
}

impl IntoAnyPass for Pipeline {
    fn as_anypass(&self) -> AnyPass {
        let p = Python::attach(|py| Py::new(py, PyPipeline(self.clone()))).unwrap();
        AnyPass::PyPipeline(p)
    }
}

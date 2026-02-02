use std::collections::HashSet;

use lunamodel_transform::{
    BasePass,
    passes::special::{AbstractPipeline, Pipeline},
};
use lunamodel_unwind::*;
use pyo3::{PyErr, PyResult, pyclass, pymethods};

use crate::transform::pass::PyPass;

#[pyclass(subclass, unsendable)]
#[derive(Debug, Clone)]
pub struct PyPipeline {
    pub(crate) p: Pipeline,
}

#[unwindable]
#[pymethods]
impl PyPipeline {
    #[new]
    #[pyo3(signature = (passes, name=None))]
    fn py_new(passes: Vec<PyPass>, name: Option<String>) -> PyResult<Self> {
        let mapped = passes
            .iter()
            .map(|y| y.as_pass())
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            p: Pipeline::new(mapped, name),
        })
    }

    #[getter]
    fn name(&self) -> String {
        return self.p.name();
    }

    #[getter]
    fn requires(&self) -> Vec<String> {
        return self.p.requires();
    }

    #[getter]
    fn satisfies(&self) -> HashSet<String> {
        return self.p.satisfies();
    }

    fn clear(&mut self) {
        self.p.clear()
    }

    fn add(&mut self, pass: PyPass) -> PyResult<()> {
        self.p.add(pass.as_pass()?);
        Ok(())
    }

    fn __len__(&self) -> usize {
        self.p.len()
    }

    #[getter]
    fn passes(&self) -> PyResult<Vec<PyPass>> {
        self.p
            .passes()
            .iter()
            .map(|p| PyPass::from_pass(p))
            .collect::<Result<Vec<_>, PyErr>>()
    }

    fn __repr__(&self) -> String {
        format!("{}", self.p)
    }
}

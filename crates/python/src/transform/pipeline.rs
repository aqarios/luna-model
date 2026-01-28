use std::collections::HashSet;

use lunamodel_unwind::*;
use lunamodel_transform::{
    BasePass,
    passes::special::{AbstractPipeline, Pipeline},
};
use pyo3::{PyResult, pyclass, pymethods};

use crate::transform::pass::PyPass;

#[pyclass(subclass, unsendable)]
#[derive(Debug, Clone)]
pub struct PyPipeline {
    pub(crate) p: Pipeline,
    passes: Vec<PyPass>,
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
            passes,
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
    fn passes(&self) -> Vec<PyPass> {
        self.passes.clone()
    }

    fn __repr__(&self) -> String {
        format!("{}", self.p)
    }
}

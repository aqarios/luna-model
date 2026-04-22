use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_transpiler::{Pipeline, PipelineStep};
use pyo3::{Py, PyAny, PyResult, Python, pyclass, pymethods};

use crate::transform::{pass::PyPass, utils::FromSteps};

#[pyclass(subclass)]
#[derive(Clone)]
pub struct PyPipeline(pub(crate) Pipeline);

impl PyPipeline {
    pub fn steps(&self) -> &[PipelineStep] {
        &self.0.steps
    }

    pub fn name(&self) -> String {
        self.0.name.clone()
    }
}

#[pymethods]
impl PyPipeline {
    #[new]
    fn new(py: Python, name: String, steps: Vec<PyPass>) -> PyResult<Self> {
        Ok(Self(Pipeline::new(
            name,
            steps
                .into_iter()
                .map(|p| p.to_step(py))
                .collect::<PyResult<_>>()?,
        )))
    }

    #[pyo3(name = "name")]
    fn pyname(&self) -> String {
        self.name()
    }

    fn requires(&self) -> Vec<String> {
        self.0.requires().collect()
    }

    fn invalidates(&self) -> Vec<String> {
        self.0.invalidates().collect()
    }

    fn provides(&self) -> Vec<String> {
        self.0.provides().collect()
    }

    fn add(&mut self, py: Python, pass: PyPass) -> PyResult<()> {
        Ok(self.0.steps.push(pass.to_step(py)?))
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn passes(&self, py: Python) -> PyResult<Vec<Py<PyAny>>> {
        Ok(self.steps().to_pypasses(py)?)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0.format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0.format(FormatOpt::Py))
    }
}

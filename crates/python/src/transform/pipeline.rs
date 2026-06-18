//! Python wrapper for reusable transformation pipelines.

use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_transpiler::{Pipeline, PipelineStep};
use pyo3::{Py, PyAny, PyResult, Python, pyclass, pymethods};

use crate::transform::{pass::PyPass, utils::FromSteps};

#[pyclass(from_py_object, subclass)]
#[derive(Clone)]
pub struct PyPipeline(pub(crate) Pipeline);

impl PyPipeline {
    /// Borrow the pipeline steps in their Rust representation.
    pub fn steps(&self) -> &[PipelineStep] {
        &self.0.steps
    }

    /// Return the configured pipeline name.
    pub fn name(&self) -> String {
        self.0.name.clone()
    }
}

#[pymethods]
impl PyPipeline {
    /// Create a named pipeline from Python-visible passes.
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

    /// Return the configured name.
    #[pyo3(name = "name")]
    fn pyname(&self) -> String {
        self.name()
    }

    /// Return the analyses required by the pipeline.
    fn requires(&self) -> Vec<String> {
        self.0.requires().collect()
    }

    /// Return the analyses invalidated by the pipeline.
    fn invalidates(&self) -> Vec<String> {
        self.0.invalidates().collect()
    }

    /// Return the analyses produced by the pipeline.
    fn provides(&self) -> Vec<String> {
        self.0.provides().collect()
    }

    /// Append one pass to the end of the pipeline.
    fn add(&mut self, py: Python, pass: PyPass) -> PyResult<()> {
        self.0.steps.push(pass.to_step(py)?);
        Ok(())
    }

    /// Remove all steps from the pipeline.
    fn clear(&mut self) {
        self.0.clear();
    }

    /// Materialize the stored steps back into Python wrapper objects.
    fn passes(&self, py: Python) -> PyResult<Vec<Py<PyAny>>> {
        Ok(self.steps().to_pypasses(py)?)
    }

    /// Format the pipeline for end-user display.
    fn __str__(&self) -> String {
        format!("{}", self.0.format(FormatOpt::Py))
    }

    /// Format the pipeline for debugging.
    fn __repr__(&self) -> String {
        format!("{:?}", self.0.format(FormatOpt::Py))
    }
}

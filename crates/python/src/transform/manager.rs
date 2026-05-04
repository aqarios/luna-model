//! Python wrapper for [`PassManager`].

use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_transpiler::PassManager;
use lunamodel_unwind::*;
use pyo3::{PyResult, Python, pyclass, pymethods};

use super::{output::PyTransformationOutput, pass::PyPass};
use crate::{PyModel, PySolution};

#[pyclass]
pub struct PyPassManager {
    /// The underlying Rust pass manager.
    pub pm: PassManager,
}

#[unwindable]
#[pymethods]
impl PyPassManager {
    /// Create a pass manager from an optional list of passes.
    ///
    /// Python passes are converted eagerly into Rust pipeline steps so runtime
    /// errors around adapter creation surface at construction time.
    #[new]
    fn new(py: Python, passes: Option<Vec<PyPass>>) -> PyResult<Self> {
        Ok(PyPassManager {
            pm: match passes {
                Some(steps) => PassManager::from_steps(
                    steps
                        .into_iter()
                        .map(|p| p.to_step(py))
                        .collect::<PyResult<_>>()?,
                ),
                None => PassManager::default(),
            },
        })
    }

    /// Append a pass to the manager.
    fn add(&mut self, py: Python, pass: PyPass) -> PyResult<()> {
        let step = pass.to_step(py)?;
        // NOTE: std::mem::take required since we use builder pattern internally => we need
        // temporary ownership.
        self.pm = std::mem::take(&mut self.pm).add_step(step);
        Ok(())
    }

    /// Run the pass manager on a deep clone of the input model.
    ///
    /// The Python binding does not mutate the caller's model in place; the
    /// returned [`PyTransformationOutput`] contains the transformed model.
    fn run(&self, model: PyModel) -> PyResult<PyTransformationOutput> {
        Ok(self.pm.run(model.m.read_arc().deep_clone())?.into())
    }

    /// Deprecated compatibility wrapper around [`PyTransformationOutput::backward`].
    fn backwards(&self, solution: PySolution, ir: &PyTransformationOutput) -> PyResult<PySolution> {
        ir.backward(solution)
    }

    /// Format the manager using Python-oriented display conventions.
    pub fn __str__(&self) -> String {
        format!("{}", self.pm.format(FormatOpt::Py))
    }

    /// Format the manager using Python-oriented debug conventions.
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.pm.format(FormatOpt::Py))
    }
}

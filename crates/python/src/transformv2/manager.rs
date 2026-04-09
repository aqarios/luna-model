use lunamodel_transpiler::PassManager;
use lunamodel_unwind::*;
use pyo3::{PyResult, Python, pyclass, pymethods};

use super::{output::PyTransformationOutput, pass::PyPass};
use crate::{PyModel, PySolution};

#[pyclass]
pub struct PyPassManager {
    pub pm: PassManager,
}

#[unwindable]
#[pymethods]
impl PyPassManager {
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

    fn add(&mut self, py: Python, pass: PyPass) -> PyResult<()> {
        let step = pass.to_step(py)?;
        // NOTE: std::mem::take required since we use builder pattern internally => we need
        // temporary ownership.
        self.pm = std::mem::take(&mut self.pm).add_step(step);
        Ok(())
    }

    fn run(&self, model: PyModel) -> PyResult<PyTransformationOutput> {
        Ok(self.pm.run(model.m.read_arc().deep_clone())?.into())
    }

    // TODO: remove once deprecated fully.
    fn backwards(&self, solution: PySolution, ir: &PyTransformationOutput) -> PyResult<PySolution> {
        ir.backward(solution)
    }

    pub fn __str__(&self) -> String {
        // TODO: move display to lunamodel_io
        // format!("{}", self.pm)
        String::from("PassManager")
    }

    // pub fn __repr__(&self) -> String {
    //     // TODO: move/overwrite Debug to/in lunamodel_io
    //     // for python specific formatting.
    //     format!("{:?}", self.pm)
    // }
}

use lunamodel_transform::PassManager;
use lunamodel_unwind::*;
use pyo3::{PyResult, pyclass, pymethods};

use super::ir::PyIR;
use super::pass::PyPass;
use crate::PySolution;
use crate::model::PyModel;

#[pyclass(unsendable)]
pub struct PyPassManager {
    pub pm: PassManager,
}

#[unwindable]
#[pymethods]
impl PyPassManager {
    #[new]
    fn new(passes: Option<Vec<PyPass>>) -> PyResult<Self> {
        let mapped = passes
            .map(|x| {
                x.into_iter()
                    .map(|y| y.as_pass())
                    .collect::<PyResult<Vec<_>>>()
            })
            .transpose()?;
        Ok(PyPassManager {
            pm: PassManager::new(mapped),
        })
    }

    fn add(&mut self, pass: PyPass) -> PyResult<()> {
        Ok(self.pm.add_pass(pass.as_pass()?))
    }

    fn run(&self, model: PyModel) -> PyResult<PyIR> {
        let ir = self.pm.run(model.m.read_arc().deep_clone())?;
        Ok(ir.into())
    }

    fn backwards(&self, solution: PySolution, ir: &PyIR) -> PyResult<PySolution> {
        Ok(self
            .pm
            .backwards(solution.s.read_arc().clone(), &ir.ir)?
            .into())
    }

    pub fn __str__(&self) -> String {
        // TODO: move display to lunamodel_io
        format!("{}", self.pm)
    }

    pub fn __repr__(&self) -> String {
        // TODO: move/overwrite Debug to/in lunamodel_io
        // for python specific formatting.
        format!("{:?}", self.pm)
    }
}

use derive_more::{Deref, DerefMut};
use lunamodel_python::{PyModel, PySolution};
use lunamodel_unwind::*;
use pyo3::prelude::{PyResult, pyclass, pymethods};

use crate::pass_manager::PassManager;

use super::AnyPass;
use super::ir::PyIR;

#[pyclass(unsendable, name = "PassManager")]
#[derive(Deref, DerefMut)]
pub struct PyPassManager(PassManager);

#[unwindable]
#[pymethods]
impl PyPassManager {
    #[new]
    #[pyo3(signature=(passes=None))]
    pub fn py_new(passes: Option<Vec<AnyPass>>) -> PyResult<Self> {
        let mapped = passes
            .map(|x| {
                x.into_iter()
                    .map(|y| y.as_pass())
                    .collect::<PyResult<Vec<_>>>()
            })
            .transpose()?;
        Ok(PyPassManager(PassManager::new(mapped)))
    }

    #[pyo3(name = "add")]
    pub fn py_add(&mut self, pass: AnyPass) -> PyResult<()> {
        Ok(self.add_pass(pass.as_pass()?))
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[pyo3(name = "run")]
    pub fn py_run(&self, model: PyModel) -> PyResult<PyIR> {
        Ok(PyIR(self.run(model.m.read_arc().deep_clone())?))
    }

    #[pyo3(name = "backwards")]
    pub fn py_backwards(&self, solution: &PySolution, ir: &PyIR) -> PySolution {
        self.backwards(solution.s.read_arc().clone(), &ir.0).into()
    }
}

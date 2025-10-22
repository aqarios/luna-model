use crate::py_bindings::py_sol::PySolution;
use crate::py_bindings::unwind;
use crate::{py_bindings::py_model::PyModel, transformations::pass_manager::PassManager};
use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;
use unwind_macros::unwindable;

use super::py_ir::PyIR;
use super::py_module::AnyPass;

#[pyclass(
    unsendable,
    name = "PassManager",
    module = "luna_model._core.transformations"
)]
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
        Ok(PyIR(self.run(model.access().deep_clone())?))
    }

    #[pyo3(name = "backwards")]
    pub fn py_backwargs(&self, solution: &PySolution, ir: &PyIR) -> PySolution {
        PySolution::new(self.backwards(solution.access().clone(), &ir.0))
    }
}

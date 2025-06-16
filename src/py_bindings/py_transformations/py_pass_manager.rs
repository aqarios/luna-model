use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

use crate::{core::ConcreteModel, py_bindings::py_model::PyModel, transformations::pass_manager::ConcretePassManager};

use super::passes::any_pass::AnyPass;
use super::py_analysis_cache::PyAnalysisCache;

// TODO: Docstrings
#[pyclass(unsendable, name = "PassManager", module = "aqmodels.transformations")]
#[derive(Deref, DerefMut)]
pub struct PyPassManager(ConcretePassManager);

#[pymethods]
impl PyPassManager {
    #[new]
    #[pyo3(signature=(passes=None))]
    pub fn py_new(passes: Option<Vec<AnyPass>>) -> Self {
        PyPassManager(ConcretePassManager::new(
            passes.map(|x| x.into_iter().map(|y| y.as_pass()).collect()),
        ))
    }

    #[pyo3(name = "add")]
    pub fn py_add(&mut self, pass: AnyPass) {
        self.add_pass(pass.as_pass());
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[pyo3(name = "run")]
    pub fn py_run(&self, model: PyModel) -> PyResult<(PyModel, PyAnalysisCache)> {
        let input = model.borrow().clone();
        let ir = self.run(input).unwrap();
        Ok((PyModel::new(ir.model), PyAnalysisCache::new(ir.cache)))
    }
}

use derive_more::{Deref, DerefMut};
use pyo3::exceptions::PyRuntimeError;
use pyo3::{create_exception, prelude::*};

use crate::transformations::errors::CompilationError as CompilationErr;
use crate::{py_bindings::py_model::PyModel, transformations::pass_manager::PassManager};

use super::py_module::AnyPass;
use super::py_analysis_cache::PyAnalysisCache;

// TODO: Docstrings
#[pyclass(unsendable, name = "PassManager", module = "aqmodels.transformations")]
#[derive(Deref, DerefMut)]
pub struct PyPassManager(PassManager);

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
    pub fn py_run(&self, model: PyModel) -> PyResult<(PyModel, PyAnalysisCache)> {
        let input = model.borrow().deep_clone();
        let ir = self.run(input)?;
        Ok((PyModel::new(ir.model), PyAnalysisCache::new(ir.cache)))
    }
}

create_exception!(aqmodels.errors, CompilationError, PyRuntimeError);

impl From<CompilationErr> for PyErr {
    fn from(value: CompilationErr) -> Self {
        CompilationError::new_err(value.to_string())
    }
}

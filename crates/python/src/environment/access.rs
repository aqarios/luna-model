use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyEnvironment;
use crate::PyVariable;

#[unwindable]
#[pymethods]
impl PyEnvironment {
    #[getter]
    fn id(&self) -> usize {
        self.env.read_arc().id()
    }

    #[getter]
    fn num_variables(&self) -> usize {
        self.env.len()
    }

    fn get_variable(&self, name: String) -> PyResult<PyVariable> {
        Ok(PyVariable::new(self.env.lookup(&name)?))
    }

    fn variables(&self) -> Vec<PyVariable> {
        self.env.vars().into_iter().map(PyVariable::new).collect()
    }

    fn __contains__(&self, varname: String) -> bool {
        self.env.contains(&varname)
    }
}

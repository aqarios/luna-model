//! Accessors for the Python `Environment` wrapper.

use lunamodel_unwind::*;
use pyo3::{PyResult, pymethods};

use super::PyEnvironment;
use crate::PyVariable;

#[unwindable]
#[pymethods]
impl PyEnvironment {
    /// Return the internal environment identifier.
    ///
    /// The identifier is assigned by the Rust core and is primarily useful when
    /// debugging environment mismatches across wrappers.
    #[getter]
    fn id(&self) -> usize {
        self.env.read_arc().id()
    }

    /// Return the number of variables currently registered in the environment.
    #[getter]
    fn num_variables(&self) -> usize {
        self.env.len()
    }

    /// Look up a variable by name and wrap the resulting `VarRef`.
    fn get_variable(&self, name: String) -> PyResult<PyVariable> {
        Ok(PyVariable::new(self.env.lookup(&name)?))
    }

    /// Materialize all variables currently stored in the environment.
    ///
    /// This eagerly wraps each `VarRef` so Python callers can iterate without
    /// holding a Rust-side read guard open.
    fn variables(&self) -> Vec<PyVariable> {
        self.env.vars().into_iter().map(PyVariable::new).collect()
    }

    /// Implement Python's `in` operator for variable-name membership tests.
    fn __contains__(&self, varname: String) -> bool {
        self.env.contains(&varname)
    }
}

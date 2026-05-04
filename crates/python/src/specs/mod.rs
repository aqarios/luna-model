//! Python wrapper for model specs.
mod access;
mod io;

use lunamodel_types::Specs;
use pyo3::pyclass;

#[pyclass]
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PyModelSpecs {
    /// Stored core specs value.
    pub s: Specs,
}

impl From<Specs> for PyModelSpecs {
    /// Wraps core specs for Python.
    fn from(s: Specs) -> Self {
        Self { s }
    }
}

impl From<PyModelSpecs> for Specs {
    /// Unwraps Python specs into the core type.
    fn from(val: PyModelSpecs) -> Self {
        val.s
    }
}

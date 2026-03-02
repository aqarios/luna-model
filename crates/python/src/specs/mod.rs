mod access;
mod io;

use lunamodel_types::Specs;
use pyo3::pyclass;

#[pyclass]
#[repr(C)]
#[derive(Clone)]
pub struct PyModelSpecs {
    pub s: Specs,
}

impl From<Specs> for PyModelSpecs {
    fn from(s: Specs) -> Self {
        Self { s }
    }
}

impl Into<Specs> for PyModelSpecs {
    fn into(self) -> Specs {
        self.s
    }
}

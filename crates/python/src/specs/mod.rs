use lunamodel_types::Specs;
use pyo3::pyclass;

#[pyclass]
#[repr(C)]
pub struct PyModelSpecs {
    pub s: Specs,
}

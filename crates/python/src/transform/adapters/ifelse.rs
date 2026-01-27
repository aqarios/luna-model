use pyo3::pyclass;

#[pyclass(unsendable)]
#[derive(Clone, Debug)]
#[repr(C)]
pub struct PyIfElsePass {}

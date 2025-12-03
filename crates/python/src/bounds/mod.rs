use lunamodel_core::prelude::LazyBounds;
use pyo3::pyclass;

#[pyclass]
#[derive(Clone)]
pub struct PyBounds(pub LazyBounds);

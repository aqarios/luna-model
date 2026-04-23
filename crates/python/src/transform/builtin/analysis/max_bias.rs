use lunamodel_python_macros::pyanalysis;
use lunamodel_transform::analysis::{MaxBias, MaxBiasAnalysis};
use pyo3::{pyclass, pymethods};

#[pyclass]
pub struct PyMaxBias(pub MaxBias);

#[pymethods]
impl PyMaxBias {
    #[getter]
    fn val(&self) -> f64 {
        self.0.val
    }
}

impl From<MaxBias> for PyMaxBias {
    fn from(v: MaxBias) -> Self {
        Self(v)
    }
}

#[pyanalysis(PyMaxBias)]
#[derive(Default)]
pub struct PyMaxBiasAnalysis(pub MaxBiasAnalysis);

#[pymethods]
impl PyMaxBiasAnalysis {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}

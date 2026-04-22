use std::collections::HashMap;

use lunamodel_python_macros::pyanalysis;
use lunamodel_transform::analysis::{MinConstraintValues, MinValueForConstraintAnalysis};
use pyo3::{pyclass, pymethods};

#[pyclass]
pub struct PyMinConstraintValues(pub MinConstraintValues);

#[pymethods]
impl PyMinConstraintValues {
    #[getter]
    fn vals(&self) -> HashMap<String, f64> {
        self.0.vals.clone()
    }
}

impl From<MinConstraintValues> for PyMinConstraintValues {
    fn from(v: MinConstraintValues) -> Self {
        Self(v)
    }
}

#[pyanalysis(PyMinConstraintValues)]
#[derive(Default)]
pub struct PyMinValueForConstraintAnalysis(pub MinValueForConstraintAnalysis);

#[pymethods]
impl PyMinValueForConstraintAnalysis {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}

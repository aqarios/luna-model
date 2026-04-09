use std::collections::HashMap;

use lunamodel_transformv2::analysis::{MinConstraintValues, MinValueInConstraintAnalysis};
use lunamodel_transpiler::AnalysisPass;
use pyo3::{Bound, pyclass, pymethods, types::PyType};

#[pyclass]
pub struct PyMinConstraintValues(pub MinConstraintValues);

#[pymethods]
impl PyMinConstraintValues {
    #[getter]
    fn vals(&self) -> HashMap<String, f64> {
        self.0.vals.clone()
    }
}

#[pyclass(subclass)]
#[derive(Default)]
pub struct PyMinValueInConstraintAnalysis(pub MinValueInConstraintAnalysis);

#[pymethods]
impl PyMinValueInConstraintAnalysis {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[classmethod]
    fn provides(_cls: &Bound<'_, PyType>) -> String {
        MinValueInConstraintAnalysis::PROVIDES.to_string()
    }
}

impl PyMinValueInConstraintAnalysis {
    pub fn to_rs(&self) -> MinValueInConstraintAnalysis {
        self.0.clone()
    }
}

use std::collections::HashMap;

use lunamodel_transformv2::analysis::{MinConstraintValues, MinValueForConstraintAnalysis};
use lunamodel_transpiler::AnalysisPass;
use pyo3::{Bound, PyResult, pyclass, pymethods, types::PyType};

use crate::{PyModel, transformv2::PyPassContext};

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
pub struct PyMinValueForConstraintAnalysis(pub MinValueForConstraintAnalysis);

#[pymethods]
impl PyMinValueForConstraintAnalysis {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[classmethod]
    fn provides(_cls: &Bound<'_, PyType>) -> String {
        MinValueForConstraintAnalysis::PROVIDES.to_string()
    }

    fn name(&self) -> String {
        self.0.name().to_string()
    }

    fn run(&self, model: PyModel, ctx: &PyPassContext) -> PyResult<PyMinConstraintValues> {
        Ok(PyMinConstraintValues(self.0.run(&model.0.m.read_arc(), &ctx.into())?))
    }

    fn requires(&self) -> Vec<String> {
        self.0.requires().to_vec()
    }

    fn __str__(&self) -> String {
        self.0.display()
    }
}

impl PyMinValueForConstraintAnalysis {
    pub fn to_rs(&self) -> MinValueForConstraintAnalysis {
        self.0.clone()
    }
}

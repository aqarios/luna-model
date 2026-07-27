//! Python wrapper for minimum-constraint-value analysis.

use std::collections::HashMap;

use lunamodel_python_macros::pyanalysis;
use lunamodel_transform::analysis::{MinConstraintValues, MinValueForConstraintAnalysis};
use lunamodel_transpiler::AnalysisPass;
use pyo3::{pyclass, pymethods};

use crate::{
    PyModel,
    bounds::BoundValue,
    transform::{PyPassContext, error::to_pyerr},
};

#[pyclass]
pub struct PyMinConstraintValues(pub MinConstraintValues);

#[pymethods]
impl PyMinConstraintValues {
    #[getter]
    fn vals(&self) -> HashMap<String, BoundValue> {
        self.0
            .vals
            .iter()
            .map(|(n, b)| (n.clone(), b.into()))
            .collect()
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

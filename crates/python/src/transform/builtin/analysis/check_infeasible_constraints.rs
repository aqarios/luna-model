//! Python wrapper for infeasible constraints checking analysis.

use lunamodel_python_macros::pyanalysis;
use lunamodel_transform::analysis::CheckInfeasibleConstraintsAnalysis;
use lunamodel_transpiler::AnalysisPass;
use pyo3::pymethods;

use crate::{
    PyModel,
    transform::{PyPassContext, error::to_pyerr},
};

#[pyanalysis]
#[derive(Default)]
pub struct PyCheckInfeasibleConstraintsAnalysis(pub CheckInfeasibleConstraintsAnalysis);

#[pymethods]
impl PyCheckInfeasibleConstraintsAnalysis {
    #[new]
    fn new() -> Self {
        Self::default()
    }
}

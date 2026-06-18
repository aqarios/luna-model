//! Python wrappers for control-flow execution plans.

use lunamodel_transpiler::ControlFlowPlan;
use pyo3::{PyResult, Python, pyclass, pymethods};

use crate::transform::utils::PipelineOrPassVec;

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct PyControlFlowPlan(pub ControlFlowPlan);

#[pymethods]
impl PyControlFlowPlan {
    #[new]
    fn new(py: Python, name: String, steps: PipelineOrPassVec) -> PyResult<Self> {
        Ok(Self(ControlFlowPlan::new(name, steps.to_steps(py)?)))
    }

    #[getter]
    fn name(&self) -> String {
        self.0.name().to_string()
    }
}

impl From<ControlFlowPlan> for PyControlFlowPlan {
    fn from(value: ControlFlowPlan) -> Self {
        Self(value)
    }
}

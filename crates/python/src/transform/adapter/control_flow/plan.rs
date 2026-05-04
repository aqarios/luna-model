//! Python wrappers for control-flow execution plans.

use lunamodel_transpiler::ControlFlowPlan;
use pyo3::{PyResult, Python, pyclass, pymethods};

use crate::transform::utils::PipelineOrPassVec;

#[pyclass]
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

    // #[getter]
    // fn steps(&self, py: Python) -> PyResult<Vec<Py<PyAny>>> {
    //     Ok(self.0.steps().to_pypasses(py)?)
    // }
}

impl From<ControlFlowPlan> for PyControlFlowPlan {
    fn from(value: ControlFlowPlan) -> Self {
        Self(value)
    }
}

//! Adapters for Python-defined control-flow passes.

use pyo3::{Bound, PyAny, pyclass, pymethods};

mod adapter;
mod plan;

pub use adapter::PyControlFlowPassAdapter;
pub use plan::PyControlFlowPlan;

#[pyclass(subclass)]
pub struct PyControlFlowPass;
#[pymethods]
impl PyControlFlowPass {
    #[new]
    #[pyo3(signature=(*args, **kwargs))]
    fn py_new(args: &Bound<'_, PyAny>, kwargs: Option<&Bound<'_, PyAny>>) -> Self {
        _ = (args, kwargs);
        Self {}
    }
}

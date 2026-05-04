//! Adapters for Python-defined analysis passes.

use pyo3::{Bound, PyAny, pyclass, pymethods};

mod adapter;
mod result;

pub use adapter::PyAnalysisPassAdapter;
pub use result::PyAnalysisPassAdapterResult;

#[pyclass(subclass)]
pub struct PyAnalysisPass;
#[pymethods]
impl PyAnalysisPass {
    #[new]
    #[pyo3(signature=(*args, **kwargs))]
    fn py_new(args: &Bound<'_, PyAny>, kwargs: Option<&Bound<'_, PyAny>>) -> Self {
        _ = (args, kwargs);
        Self {}
    }
}

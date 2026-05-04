//! Adapters for Python-defined composite passes.

use pyo3::{Bound, PyAny, pyclass, pymethods};

mod adapter;

pub use adapter::PyCompositePassAdapter;

#[pyclass(subclass)]
pub struct PyCompositePass;
#[pymethods]
impl PyCompositePass {
    #[new]
    #[pyo3(signature=(*args, **kwargs))]
    fn py_new(args: &Bound<'_, PyAny>, kwargs: Option<&Bound<'_, PyAny>>) -> Self {
        _ = (args, kwargs);
        Self {}
    }
}

use pyo3::{Bound, PyAny, pyclass, pymethods};

mod adapter;
mod artifact;
mod envelope;

pub use adapter::PyTransformationPassAdapter;

#[pyclass(subclass)]
pub struct PyTransformationPass;
#[pymethods]
impl PyTransformationPass {
    #[new]
    #[pyo3(signature=(*args, **kwargs))]
    fn py_new(args: &Bound<'_, PyAny>, kwargs: Option<&Bound<'_, PyAny>>) -> Self {
        _ = (args, kwargs);
        Self {}
    }
}

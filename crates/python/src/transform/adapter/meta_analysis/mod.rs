use pyo3::{Bound, PyAny, pyclass, pymethods};

mod adapter;
mod result;
mod view;

pub use adapter::PyMetaAnalysisPassAdapter;
pub use result::PyMetaAnalysisPassAdapterResult;

#[pyclass(subclass)]
pub struct PyMetaAnalysisPass;
#[pymethods]
impl PyMetaAnalysisPass {
    #[new]
    #[pyo3(signature=(*args, **kwargs))]
    fn py_new(args: &Bound<'_, PyAny>, kwargs: Option<&Bound<'_, PyAny>>) -> Self {
        _ = (args, kwargs);
        Self {}
    }
}

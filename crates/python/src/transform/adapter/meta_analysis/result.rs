use pyo3::{Py, PyAny};

#[derive(Debug)]
pub struct PyMetaAnalysisPassAdapterResult(pub Py<PyAny>);

use pyo3::{Py, PyAny};

#[derive(Debug)]
pub struct PyAnalysisPassAdapterResult(pub Py<PyAny>);

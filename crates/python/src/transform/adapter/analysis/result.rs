//! Wrapper for Python analysis results cached in the Rust analysis manager.

use pyo3::{Py, PyAny};

#[derive(Debug)]
pub struct PyAnalysisPassAdapterResult(pub Py<PyAny>);

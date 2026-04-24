//! Wrapper for Python meta-analysis results cached in the Rust analysis manager.

use pyo3::{Py, PyAny};

#[derive(Debug)]
pub struct PyMetaAnalysisPassAdapterResult(pub Py<PyAny>);

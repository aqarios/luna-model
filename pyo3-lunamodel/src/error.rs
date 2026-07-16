//! Error conversion at the PyO3/extension boundary.

use pyo3::{PyErr, exceptions::PyRuntimeError};

/// Converts a LunaModel C-API error into a Python exception.
pub fn into_pyerr(error: rs_lunamodel::Error) -> PyErr {
    PyRuntimeError::new_err(error.to_string())
}

/// Converts a Python exception into an extension-side LunaModel error.
///
/// The original Python exception remains represented in its rendered message;
/// the C ABI intentionally does not transport Python-owned exception objects.
pub fn from_pyerr(error: PyErr) -> rs_lunamodel::Error {
    rs_lunamodel::Error::computation(error.to_string())
}

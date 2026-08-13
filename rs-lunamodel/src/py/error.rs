//! Error conversion at the PyO3/extension boundary.

use pyo3::{PyErr, Python, exceptions::PyRuntimeError, types::PyAnyMethods};

/// Converts a LunaModel C-API error into a Python exception.
///
/// Raises `luna_model.errors.LunaModelError` — the same base class native LunaModel failures
/// raise — rather than a generic `RuntimeError`, so Python callers can catch a plugin's failures
/// the same way they catch the core library's (`except LunaModelError`), matching what an
/// in-process (native) pass failure would have raised. Falls back to `PyRuntimeError` only if the
/// `luna_model.errors.LunaModelError` lookup itself fails (e.g. `luna_model` isn't importable),
/// so reporting an error can never itself panic.
pub fn into_pyerr(error: crate::Error) -> PyErr {
    let message = error.to_string();
    Python::attach(|py| {
        crate::py::luna_model(py)
            .and_then(|m| m.getattr("errors"))
            .and_then(|errors| errors.getattr("LunaModelError"))
            .and_then(|exc_type| exc_type.call1((message.clone(),)))
            .map(PyErr::from_value)
            .unwrap_or_else(|_| PyRuntimeError::new_err(message))
    })
}

/// Converts a Python exception into an extension-side LunaModel error.
///
/// The original Python exception remains represented in its rendered message;
/// the C ABI intentionally does not transport Python-owned exception objects.
pub fn from_pyerr(error: PyErr) -> crate::Error {
    crate::Error::computation(error.to_string())
}

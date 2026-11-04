//! Error conversion at the PyO3/extension boundary.

use pyo3::{PyErr, Python, exceptions::PyRuntimeError, types::PyAnyMethods};

/// Converts a LunaModel C-API error into a Python exception.
///
/// Raises the class luna-model itself raises for the error's kind (e.g. `ComputationError`,
/// `ModelVtypeError`; `IndexError` for an out-of-bounds index), so Python callers catch a
/// plugin's failures exactly like the core library's. The class comes from the installed
/// luna-model (`luna_model.errors._error_for_kind`), the single kind -> exception mapping, so
/// kinds added to luna-model later map correctly without rebuilding the extension. Unknown kinds
/// raise `LunaModelError`, as does every error on a luna-model without that mapping (before
/// 0.8.0a2). Falls back to `PyRuntimeError` only if the lookup itself fails (e.g. `luna_model`
/// isn't importable), so reporting an error can never itself panic.
pub fn into_pyerr(error: crate::Error) -> PyErr {
    let kind = error.kind_tag.clone().unwrap_or_default();
    // The bare message, like a native error's; `Display` would prefix the status.
    let message = if error.message.is_empty() {
        error.to_string()
    } else {
        error.message
    };
    Python::attach(|py| {
        crate::py::luna_model(py)
            .and_then(|m| m.getattr("errors"))
            .and_then(|errors| match errors.getattr("_error_for_kind") {
                Ok(error_for_kind) => error_for_kind.call1((kind, message.clone())),
                Err(_) => errors.getattr("LunaModelError")?.call1((message.clone(),)),
            })
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

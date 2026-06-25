#[macro_use]
mod macros;

mod bounds;
mod constraint;
mod constraint_collection;
mod environment;
mod expression;
mod model;
mod pass_ctx;
mod sol;
mod transpiler;
mod types;
mod utils;
mod variable;

pub mod prelude;

use pyo3::exceptions::PyImportError;
use pyo3::prelude::{Bound, Py, PyAnyMethods, PyModule, PyResult, Python};
use std::sync::LazyLock;

pub use luna_model::core;
pub use luna_model::python::PyExprContent;

/// Capsule ABI this extension was compiled against.
///
/// Taken from the same `luna-model` build the extension links, so it is exactly
/// the layout the `from_capsule`/`to_capsule` impls assume. It must match the
/// `luna_model` host at runtime; [`check_host_abi`] enforces that at import.
const CAPSULE_ABI: u32 = luna_model::python::ffi::CAPSULE_ABI;

static LUNA_MODEL: LazyLock<PyResult<Py<PyModule>>> = LazyLock::new(|| {
    Python::attach(|py| {
        let module = PyModule::import(py, "luna_model")?;
        check_host_abi(&module)?;
        Ok(module.unbind())
    })
});

/// Rejects a `luna_model` host whose capsule ABI differs from this extension's,
/// turning a would-be segfault at the first capsule crossing into a clear
/// `ImportError` the moment the host is first used.
fn check_host_abi(luna_model: &Bound<'_, PyModule>) -> PyResult<()> {
    // Hosts predating capsule versioning do not export `__capsule_abi__`; they
    // are the ABI-1 baseline, so a missing attribute is treated as 1.
    let host_abi = match luna_model
        .getattr("_lm")
        .and_then(|lm| lm.getattr("__capsule_abi__"))
    {
        Ok(value) => value.extract::<u32>()?,
        Err(_) => 1,
    };
    abi_mismatch(CAPSULE_ABI, host_abi).map_or(Ok(()), |msg| Err(PyImportError::new_err(msg)))
}

/// Returns a descriptive error message when `extension_abi` and `host_abi` are
/// incompatible, or `None` when they match. Pure so it is unit-testable without
/// a Python interpreter.
fn abi_mismatch(extension_abi: u32, host_abi: u32) -> Option<String> {
    if extension_abi == host_abi {
        return None;
    }
    Some(format!(
        "luna_model capsule ABI mismatch: this extension was built against \
         luna-model capsule ABI v{extension_abi}, but the imported `luna_model` \
         exposes capsule ABI v{host_abi}. The two exchange Rust objects through \
         raw-pointer capsules whose memory layout differs across these ABIs, so \
         using them together is unsound. Rebuild this extension against a \
         `pyo3-lunamodel` release matching the installed `luna_model` (or update \
         `luna_model` to match the extension)."
    ))
}

pub(crate) fn luna_model(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    LUNA_MODEL
        .as_ref()
        .map(|m| m.bind(py).clone())
        .map_err(|e| e.clone_ref(py))
}

#[cfg(test)]
mod tests {
    use super::abi_mismatch;

    #[test]
    fn matching_abi_is_accepted() {
        assert!(abi_mismatch(1, 1).is_none());
    }

    #[test]
    fn mismatched_abi_reports_both_versions() {
        let msg = abi_mismatch(2, 1).expect("a mismatch must be reported");
        assert!(msg.contains("v2"), "message should name the extension ABI: {msg}");
        assert!(msg.contains("v1"), "message should name the host ABI: {msg}");
    }
}

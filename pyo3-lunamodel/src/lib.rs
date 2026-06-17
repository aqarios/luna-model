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

use pyo3::prelude::{Bound, Py, PyModule, PyResult, Python};
use std::sync::LazyLock;

pub use lunamodel::core;
pub use lunamodel::python::PyExprContent;

static LUNA_MODEL: LazyLock<PyResult<Py<PyModule>>> =
    LazyLock::new(|| Python::attach(|py| Ok(PyModule::import(py, "luna_model")?.unbind())));

pub(crate) fn luna_model(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    LUNA_MODEL
        .as_ref()
        .map(|m| m.bind(py).clone())
        .map_err(|e| e.clone_ref(py))
}

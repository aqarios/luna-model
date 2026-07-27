//! Python-facing pass execution context.

use pyo3::prelude::*;

use crate::plugin::PassContext;

/// Python wrapper for a transpiler `PassContext` — the read-only view of analyses a pass may read
/// during `forward`.
///
/// Mirrors the host's own read-only context: a `PassContext()` constructed in Python is empty, and
/// a populated one is supplied by the host during a `PassManager` run (a pass reaches results
/// through `rs_lunamodel::plugin::PassContext::{get_analysis, require_analysis}`). Present so a
/// plugin pass's `forward` has the same `(model, ctx)` surface as a native pass.
#[pyclass(name = "PassContext")]
pub struct PyPassContext(PassContext);

#[pymethods]
impl PyPassContext {
    #[new]
    fn new() -> Self {
        Self(PassContext::detached())
    }
}

impl PyPassContext {
    /// Borrows the underlying context.
    pub fn inner(&self) -> &PassContext {
        &self.0
    }
}

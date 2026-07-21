//! The plugin-side mirror of the transpiler's `PassContext`.

use lunamodel_cext_vtable::transpiler::LmPassContext;

/// Per-pass execution context lent to [`forward`](super::TransformationPass::forward).
///
/// Mirrors `lunamodel_transpiler::PassContext`. It borrows the host's context for the duration of
/// the call only. Analysis accessors are added as the corresponding C-API surface lands; for now
/// it is an opaque borrow that keeps the `forward` signature identical to a native pass.
pub struct PassContext {
    _raw: *const LmPassContext,
}

impl PassContext {
    pub(crate) fn from_raw(raw: *const LmPassContext) -> Self {
        Self { _raw: raw }
    }

    /// Creates a detached context with no analyses.
    ///
    /// For unit-testing a pass's `forward` without a running host. Since the context currently
    /// exposes no analysis accessors, a detached one behaves like an empty context.
    pub fn new() -> Self {
        Self {
            _raw: std::ptr::null(),
        }
    }
}

impl Default for PassContext {
    fn default() -> Self {
        Self::new()
    }
}

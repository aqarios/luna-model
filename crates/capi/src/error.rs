//! Structured error state shared by the host and by host-side plugin adapters.
//!
//! [`LmStatus`](crate::LmStatus) is intentionally a small, fixed set of coarse codes — it is not
//! meant to carry domain detail. [`RichError`] is the structured error that rides alongside it:
//! an open string `kind` tag (so plugins can mint their own, the way analysis/artifact type tags
//! already work, without an ABI version bump), a message, and an optional cause chain.
//!
//! This lives in `lunamodel-capi` (rather than `lunamodel-capi-host`, where the rest of the error
//! handling lives) because both `lunamodel-capi-host` and `lunamodel-capi-plugin` need to read and
//! write the same thread-local: the host sets it from native `LunaModelError`s, and the
//! plugin/extension adapters set it from a plugin's own reported failure so it can be folded into
//! the very same transpiler error chain a native pass failure would produce.

use std::cell::RefCell;

/// A structured error crossing the C ABI as `LmError`.
#[derive(Debug, Clone)]
pub struct RichError {
    /// An open, stable string tag identifying the error's origin/category, e.g.
    /// `"variable_not_existing"` or a plugin-defined `"myplugin::infeasible_subproblem"`.
    pub kind: String,
    /// A human-readable message.
    pub message: String,
    /// The error that caused this one, if any.
    pub cause: Option<Box<RichError>>,
}

impl RichError {
    /// Creates a new, causeless rich error.
    pub fn new(kind: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            message: message.into(),
            cause: None,
        }
    }

    /// Attaches a cause.
    #[must_use]
    pub fn caused_by(mut self, cause: RichError) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }
}

impl std::fmt::Display for RichError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(cause) = &self.cause {
            write!(f, " - caused by: {cause}")?;
        }
        Ok(())
    }
}

impl std::error::Error for RichError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause
            .as_deref()
            .map(|c| c as &(dyn std::error::Error + 'static))
    }
}

thread_local! {
    static LAST_ERROR: RefCell<Option<RichError>> = const { RefCell::new(None) };
}

/// Records `err` as the current thread's last structured error.
pub fn set_last(err: RichError) {
    LAST_ERROR.with(|slot| *slot.borrow_mut() = Some(err));
}

/// Clears the current thread's last structured error.
pub fn clear_last() {
    LAST_ERROR.with(|slot| *slot.borrow_mut() = None);
}

/// Returns a clone of the current thread's last structured error, if any.
pub fn last() -> Option<RichError> {
    LAST_ERROR.with(|slot| slot.borrow().clone())
}

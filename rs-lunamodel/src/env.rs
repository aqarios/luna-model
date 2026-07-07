//! Owning wrapper over a LunaModel environment handle.

use lunamodel_cext_vtable::LmEnv;

use crate::api::api;

/// Owning handle to a LunaModel environment (frees on drop).
///
/// The shared context that resolves the variables referenced by an
/// [`crate::Expression`].
pub struct Env {
    handle: *mut LmEnv,
}

impl Env {
    /// Creates a fresh, empty environment.
    pub fn new() -> Self {
        // `env_default` returns a non-null handle.
        Env {
            handle: (api().env_default)(),
        }
    }

    /// Adopts ownership of a raw handle produced by the C ABI.
    ///
    /// # Safety
    /// `handle` must be non-null, originate from this library, and not be owned
    /// elsewhere.
    pub(crate) fn from_raw(handle: *mut LmEnv) -> Self {
        Env { handle }
    }

    /// Borrows the raw handle for passing to C ABI calls.
    pub(crate) fn as_ptr(&self) -> *const LmEnv {
        self.handle
    }
}

// Lets `out!` convert a raw `*mut LmEnv` out value straight into `Env` via
// `lm_try_into()`, matching the `RawHandle`-derived wrappers.
impl crate::traits::LmTryFrom<*mut LmEnv> for Env {
    fn lm_try_from(handle: *mut LmEnv) -> crate::Result<Self> {
        Ok(Self::from_raw(handle))
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, env_free)
        }
    }
}

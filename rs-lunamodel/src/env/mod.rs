//! Owning wrapper over a LunaModel env handle.
//!
//! Mirrors the Rust `Env`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Env` still return `Result` here.

mod access;
mod modification;

use lunamodel_cext_vtable::LmEnv;
use rs_lunamodel_derive::RawHandle;

use crate::{ContentEquality, Result, Serialization, TryClone, TryDeepClone, api::api};

/// Owning handle to a LunaModel env (frees on drop).
#[derive(RawHandle)]
pub struct Env {
    handle: *mut LmEnv,
}

impl Drop for Env {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, env_free)
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::from_raw((api().env_default)())
    }
}

impl Env {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TryClone for Env {
    fn try_clone(&self) -> Result<Self> {
        out!(env_clone(@self))
    }
}

impl TryDeepClone<'_> for Env {
    type Target = ();
    fn try_deep_clone(&self, _: Self::Target) -> Result<Self> {
        out!(env_deep_clone(@self))
    }
}

impl ContentEquality for Env {
    fn equal_contents(&self, other: &Self) -> Result<bool> {
        out!(env_equal_contents(@self, @other))
    }
}

impl Serialization<'_> for Env {
    type Payload = ();

    fn serialize(&self) -> Result<Vec<u8>> {
        out!(env_serialize(@self))
    }
    fn deserialize(data: &[u8], _: Self::Payload) -> Result<Self> {
        out!(env_deserialize(@data, data.len()))
    }
}

impl std::fmt::Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArcEnv")
            .field("id", &self.id().ok())
            .field("len", &self.len().ok())
            .finish()
    }
}

// SAFETY: every operation on an LmEnv is synchronized by the host.
unsafe impl Send for Env {}
unsafe impl Sync for Env {}

//! Owning wrapper over a LunaModel cc handle.
//!
//! Mirrors the Rust `ConstraintCollection`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `ConstraintCollection` still return `Result` here.

mod access;
mod editor;
mod modification;

use lunamodel_capi::LmCC;
use rs_lunamodel_derive::RawHandle;

use crate::{
    Constraint, ContentEquality, Env, Result, Serialization, TryClone, TryDeepClone, api::api,
};

pub use editor::ConstraintCollectionEditor;

/// Owning handle to a LunaModel cc (frees on drop).
#[derive(RawHandle)]
pub struct ConstraintCollection {
    handle: *mut LmCC,
}

impl Default for ConstraintCollection {
    fn default() -> Self {
        Self::from_raw((api().cc_default)())
    }
}

impl ConstraintCollection {
    pub fn new() -> Self {
        Self::from_raw((api().cc_new)())
    }
}

impl TryClone for ConstraintCollection {
    fn try_clone(&self) -> Result<Self> {
        out!(cc_clone(@self))
    }
}

impl<'a> TryDeepClone<'a> for ConstraintCollection {
    type Target = &'a Env;

    fn try_deep_clone(&self, env: Self::Target) -> Result<Self> {
        out!(cc_deep_clone(@self, @env))
    }
}

impl PartialEq for ConstraintCollection {
    fn eq(&self, other: &Self) -> bool {
        out_or!(cc_eq(@self, @other), false)
    }
}

impl ContentEquality for ConstraintCollection {
    fn equal_contents(&self, other: &Self) -> Result<bool> {
        out!(cc_equal_contents(@self, @other))
    }
}

impl Serialization<'_> for ConstraintCollection {
    type Payload = Env;

    fn serialize(&self) -> Result<Vec<u8>> {
        out!(cc_serialize(@self))
    }

    fn deserialize(data: &[u8], env: Self::Payload) -> Result<Self> {
        out!(cc_deserialize(data.as_ptr(), data.len(), @env))
    }
}

impl TryFrom<indexmap::IndexMap<String, Constraint>> for ConstraintCollection {
    type Error = crate::Error;

    fn try_from(value: indexmap::IndexMap<String, Constraint>) -> Result<Self> {
        let mut collection = Self::new();
        for (name, constraint) in value {
            collection.add_constraint(&constraint, Some(&name))?;
        }
        Ok(collection)
    }
}

impl std::fmt::Debug for ConstraintCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstraintCollection")
            .field("names", &self.names().ok())
            .finish()
    }
}

impl Drop for ConstraintCollection {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, cc_free)
        }
    }
}

// SAFETY: every operation on an LmCC is synchronized by the host storage.
unsafe impl Send for ConstraintCollection {}
unsafe impl Sync for ConstraintCollection {}

//! Owning wrapper over a LunaModel expression handle.
//!
//! Mirrors the Rust `Expression`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Expression` still return `Result` here.

mod access;
mod arithmetic;
mod creation;
mod general;

use crate::{
    Env,
    api::api,
    traits::{ContentEquality, Serialization, TryClone, TryDeepClone},
};

use lunamodel_cext_vtable::LmExpr;
use rs_lunamodel_derive::RawHandle;

/// Owning handle to a LunaModel expression (frees on drop).
#[derive(RawHandle)]
pub struct Expression {
    handle: *mut LmExpr,
}

impl Drop for Expression {
    fn drop(&mut self) {
        free!(self, expr_free)
    }
}

impl Expression {
    /// Clones into a new handle sharing the environment.
    ///
    /// Named `try_clone` because the fallible C call cannot satisfy `Clone`.
    pub fn try_clone(&self) -> crate::Result<Self> {
        <Self as TryClone>::try_clone(self)
    }

    /// Deep-clones into an independent expression rebound to `env`.
    pub fn deep_clone(&self, env: &Env) -> crate::Result<Self> {
        <Self as TryDeepClone>::try_deep_clone(self, env)
    }

    /// Content equality, ignoring environment identity.
    pub fn equal_contents(&self, other: &Self) -> crate::Result<bool> {
        <Self as ContentEquality>::equal_contents(self, other)
    }

    /// Serializes the expression to bytes.
    pub fn serialize(&self) -> crate::Result<Vec<u8>> {
        <Self as Serialization>::serialize(self)
    }

    /// Deserializes an expression from bytes, rebinding it to `env`.
    pub fn deserialize(data: &[u8], env: &Env) -> crate::Result<Self> {
        <Self as Serialization>::deserialize(data, env)
    }
}

impl Default for Expression {
    /// Creates an empty expression with a fresh environment.
    fn default() -> Self {
        Self::from_raw((api().expr_default)())
    }
}

impl PartialEq for Expression {
    /// Structural equality including environment identity.
    ///
    /// An FFI failure is treated as "not equal".
    fn eq(&self, other: &Self) -> bool {
        out_or!(expr_eq(@self, @other), false)
    }
}

impl TryClone for Expression {
    fn try_clone(&self) -> crate::Result<Self> {
        out!(expr_clone(@self))
    }
}

impl<'e> TryDeepClone<'e> for Expression {
    type Target = &'e Env;

    fn try_deep_clone(&self, env: Self::Target) -> crate::Result<Self> {
        out!(expr_deep_clone(@self, @env))
    }
}

impl ContentEquality for Expression {
    fn equal_contents(&self, other: &Self) -> crate::Result<bool> {
        out!(expr_equal_contents(@self, @other))
    }
}

impl<'s> Serialization<'s> for Expression {
    type Payload = &'s Env;

    fn serialize(&self) -> crate::Result<Vec<u8>> {
        out!(expr_serialize(@self))
    }

    fn deserialize(data: &[u8], env: Self::Payload) -> crate::Result<Self> {
        out!(expr_deserialize(@data, data.len(), @env))
    }
}

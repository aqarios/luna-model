//! Owning wrapper over a LunaModel expression handle.
//!
//! Mirrors the Rust `Expression`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Expression` still return `Result` here (the
//! deliberate analog of `cxx-lunamodel` throwing on a non-`Ok` status).
//!

mod access;
mod arithmetic;
mod creation;

use crate::{
    Env,
    api::api,
    error::check,
    traits::{ContentEquality, Serialization, TryClone, TryDeepClone},
};

use lunamodel_cext_vtable::{LmBuffer, LmExpr};

/// Owning handle to a LunaModel expression (frees on drop).
pub struct Expression {
    handle: *mut LmExpr,
}

impl Expression {
    /// Wraps a raw handle owned by the caller.
    pub(crate) fn from_raw(handle: *mut LmExpr) -> Self {
        Expression { handle }
    }

    /// Borrows the raw handle for C interop.
    pub fn as_ptr(&self) -> *const LmExpr {
        self.handle
    }

    /// Clones into a new handle sharing the environment (Rust `Clone`).
    ///
    /// Named `try_clone` because the fallible C call cannot satisfy `Clone`.
    pub fn try_clone(&self) -> crate::Result<Self> {
        <Self as TryClone>::try_clone(self)
    }

    /// Deep-clones into an independent expression rebound to `env`
    /// (Rust `expr.deep_clone(env)`).
    pub fn deep_clone(&self, env: &Env) -> crate::Result<Self> {
        <Self as TryDeepClone>::try_deep_clone(self, env)
    }

    /// Content equality, ignoring environment identity
    /// (Rust `expr.equal_contents(&other)`).
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
    /// Creates an empty expression with a fresh environment
    /// (Rust `Expression::default()`).
    fn default() -> Self {
        Self::from_raw((api().expr_default)())
    }
}

impl PartialEq for Expression {
    /// Structural equality including environment identity (Rust `PartialEq`).
    ///
    /// An FFI failure is treated as "not equal".
    fn eq(&self, other: &Self) -> bool {
        let mut out = false;
        unsafe { (api().expr_eq)(self.handle, other.handle, &mut out) };
        out
    }
}

impl Drop for Expression {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            // SAFETY: `handle` was produced by this library and is freed once.
            unsafe { (api().expr_free)(self.handle) };
        }
    }
}

impl TryClone for Expression {
    fn try_clone(&self) -> crate::Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_clone)(self.handle, &mut out) })?;
        Ok(Self::from_raw(out))
    }
}

impl TryDeepClone for Expression {
    type Target = Env;

    fn try_deep_clone(&self, env: &Self::Target) -> crate::Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_deep_clone)(self.handle, env.as_ptr(), &mut out) })?;
        Ok(Self::from_raw(out))
    }
}

impl ContentEquality for Expression {
    fn equal_contents(&self, other: &Self) -> crate::Result<bool> {
        let mut out = false;
        check(unsafe { (api().expr_equal_contents)(self.handle, other.handle, &mut out) })?;
        Ok(out)
    }
}

impl Serialization for Expression {
    type Payload = Env;

    fn serialize(&self) -> crate::Result<Vec<u8>> {
        let mut buf = LmBuffer {
            data: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        };
        check(unsafe { (api().expr_serialize)(self.handle, &mut buf) })?;
        let bytes = if buf.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(buf.data, buf.len) }.to_vec()
        };
        unsafe { (api().buffer_free)(buf) };
        Ok(bytes)
    }

    fn deserialize(data: &[u8], env: &Self::Payload) -> crate::Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe {
            (api().expr_deserialize)(data.as_ptr(), data.len(), env.as_ptr(), &mut out)
        })?;
        Ok(Self::from_raw(out))
    }
}

//! Owning wrapper over a LunaModel higher_order handle.
//!
//! Mirrors the Rust `HigherOrder`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `HigherOrder` still return `Result` here.

use std::ops::{Mul, Neg};

use lunamodel_cext_vtable::LmHigherOrder;
use rs_lunamodel_derive::RawHandle;

use crate::{Result, TryClone, api::api, traits::TryAddAssign};

/// Owning handle to a LunaModel higher_order (frees on drop).
#[derive(RawHandle)]
pub struct HigherOrder {
    handle: *mut LmHigherOrder,
}

impl Drop for HigherOrder {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, higher_order_free)
        }
    }
}

impl PartialEq for HigherOrder {
    /// Structural equality including environment identity.
    ///
    /// An FFI failure is treated as "not equal".
    fn eq(&self, other: &Self) -> bool {
        out_or!(higher_order_eq(@self, @other), false)
    }
}

impl Default for HigherOrder {
    fn default() -> Self {
        Self::from_raw((api().higher_order_default)())
    }
}

impl HigherOrder {
    pub fn with_capacity(cap: usize) -> Self {
        Self::from_raw((api().higher_order_with_capacity)(cap))
    }
}

impl TryClone for HigherOrder {
    fn try_clone(&self) -> Result<Self> {
        out!(higher_order_clone(@self))
    }
}

impl HigherOrder {
    pub fn len(&self) -> Result<usize> {
        out!(higher_order_len(@self))
    }

    pub fn is_zero(&self) -> Result<bool> {
        out!(higher_order_is_zero(@self))
    }

    pub fn is_empty(&self) -> Result<bool> {
        out!(higher_order_is_empty(@self))
    }

    pub fn degree(&self) -> Result<usize> {
        out!(higher_order_degree(@self))
    }

    pub fn items(&self) -> Result<Vec<(Vec<u32>, f64)>> {
        out!(higher_order_items(@self))
    }

    pub fn index(&self, indices: &[u32]) -> Result<f64> {
        out!(higher_order_index(
            @self,
            indices.as_ptr(),
            indices.len()
        ))
    }

    pub fn set_bias(&mut self, indices: &[u32], bias: f64) -> Result<()> {
        check!(higher_order_set_bias(
            @mut self,
            indices.as_ptr(),
            indices.len(),
            bias
        ))
    }
}

impl Neg for HigherOrder {
    type Output = Result<Self>;

    fn neg(self) -> Self::Output {
        out!(higher_order_neg(@self))
    }
}

impl Mul<f64> for HigherOrder {
    type Output = Result<Self>;

    fn mul(mut self, rhs: f64) -> Self::Output {
        check!(higher_order_mul_bias(@mut self, rhs))?;
        Ok(self)
    }
}

impl TryAddAssign<(&[u32], f64)> for HigherOrder {
    fn try_add_assign(&mut self, rhs: (&[u32], f64)) -> Result<()> {
        let (indices, bias) = rhs;
        check!(higher_order_add_entry(@mut self, indices.as_ptr(), indices.len(), bias))
    }
}

impl TryAddAssign<HigherOrder> for HigherOrder {
    fn try_add_assign(&mut self, rhs: HigherOrder) -> Result<()> {
        check!(higher_order_add_higher_order(@mut self, @rhs))
    }
}

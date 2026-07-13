//! Owning wrapper over a LunaModel linear handle.
//!
//! Mirrors the Rust `Linear`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Linear` still return `Result` here.

use std::ops::{Index, IndexMut, Mul, Neg};

use lunamodel_cext_vtable::LmLinear;
use rs_lunamodel_derive::RawHandle;

use crate::{
    Result, TryClone,
    api::api,
    traits::{TryAddAssign, TryMulAssign},
};

/// Owning handle to a LunaModel linear (frees on drop).
#[derive(RawHandle)]
pub struct Linear {
    handle: *mut LmLinear,
}

impl Drop for Linear {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, linear_free)
        }
    }
}

impl PartialEq for Linear {
    /// Structural equality including environment identity.
    ///
    /// An FFI failure is treated as "not equal".
    fn eq(&self, other: &Self) -> bool {
        out_or!(linear_eq(@self, @other), false)
    }
}

impl Default for Linear {
    fn default() -> Self {
        Self::from_raw((api().linear_default)())
    }
}

impl TryClone for Linear {
    fn try_clone(&self) -> Result<Self> {
        out!(linear_clone(@self))
    }
}

impl Linear {
    pub fn for_var(var: u32, bias: f64) -> Result<Self> {
        out!(linear_for_var(var, bias))
    }

    pub fn len(&self) -> Result<usize> {
        out!(linear_len(@self))
    }

    pub fn is_zero(&self) -> Result<bool> {
        out!(linear_is_zero(@self))
    }

    pub fn is_empty(&self) -> Result<bool> {
        out!(linear_is_empty(@self))
    }

    pub fn items(&self) -> Result<Vec<(u32, f64)>> {
        out!(linear_items(@self))
    }

    pub fn push_back(&mut self, var: u32, bias: f64) -> Result<&mut Self> {
        check!(linear_add_entry(@mut self, var, bias))?;
        Ok(self)
    }

    pub fn index(&self, index: u32) -> Result<f64> {
        out!(linear_index(@self, index))
    }

    pub fn set_bias(&mut self, index: u32, bias: f64) -> Result<()> {
        check!(linear_set_bias(@mut self, index, bias))
    }
}

impl TryAddAssign<(u32, f64)> for Linear {
    fn try_add_assign(&mut self, rhs: (u32, f64)) -> Result<()> {
        let (val, bias) = rhs;
        check!(linear_add_entry(@mut self, val, bias))
    }
}

impl TryAddAssign<&Linear> for Linear {
    fn try_add_assign(&mut self, rhs: &Linear) -> Result<()> {
        check!(linear_add_linear(@mut self, @rhs))
    }
}

impl TryMulAssign<f64> for Linear {
    fn try_mul_assign(&mut self, rhs: f64) -> Result<()> {
        check!(linear_mul_bias(@mut self, rhs))
    }
}

impl Mul<f64> for Linear {
    type Output = Result<Self>;

    fn mul(mut self, rhs: f64) -> Self::Output {
        check!(linear_mul_bias(@mut self, rhs))?;
        Ok(self)
    }
}

impl Neg for Linear {
    type Output = Result<Self>;

    fn neg(self) -> Self::Output {
        out!(linear_neg(@self))
    }
}

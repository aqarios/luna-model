//! Owning wrapper over a LunaModel quadratic handle.
//!
//! Mirrors the Rust `Quadratic`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Quadratic` still return `Result` here.

use std::ops::{Mul, Neg};

use lunamodel_cext_vtable::LmQuadratic;
use rs_lunamodel_derive::RawHandle;

use crate::{Result, TryClone, api::api, traits::TryAddAssign};

/// Owning handle to a LunaModel quadratic (frees on drop).
#[derive(RawHandle)]
pub struct Quadratic {
    handle: *mut LmQuadratic,
}

impl Drop for Quadratic {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, quadratic_free)
        }
    }
}

impl PartialEq for Quadratic {
    /// Structural equality including environment identity.
    ///
    /// An FFI failure is treated as "not equal".
    fn eq(&self, other: &Self) -> bool {
        out_or!(quadratic_eq(@self, @other), false)
    }
}

impl Default for Quadratic {
    fn default() -> Self {
        Self::from_raw((api().quadratic_default)())
    }
}

impl TryClone for Quadratic {
    fn try_clone(&self) -> Result<Self> {
        out!(quadratic_clone(@self))
    }
}

impl Quadratic {
    pub fn len(&self) -> Result<usize> {
        out!(quadratic_len(@self))
    }

    pub fn is_zero(&self) -> Result<bool> {
        out!(quadratic_is_zero(@self))
    }

    pub fn is_empty(&self) -> Result<bool> {
        out!(quadratic_is_empty(@self))
    }

    pub fn items(&self) -> Result<Vec<(u32, u32, f64)>> {
        out!(quadratic_items(@self))
    }

    pub fn index(&self, u: u32, v: u32) -> Result<f64> {
        out!(quadratic_index(@self, u, v))
    }

    pub fn set_bias(&mut self, u: u32, v: u32, bias: f64) -> Result<()> {
        check!(quadratic_set_bias(@mut self, u, v, bias))
    }
}

impl TryAddAssign<(u32, u32, f64)> for Quadratic {
    fn try_add_assign(&mut self, rhs: (u32, u32, f64)) -> Result<()> {
        let (u, v, bias) = rhs;
        check!(quadratic_add_entry(@mut self, u, v, bias))
    }
}

impl TryAddAssign<Quadratic> for Quadratic {
    fn try_add_assign(&mut self, rhs: Quadratic) -> Result<()> {
        check!(quadratic_add_quadratic(@mut self, @rhs))
    }
}

impl TryAddAssign<&Quadratic> for Quadratic {
    fn try_add_assign(&mut self, rhs: &Quadratic) -> Result<()> {
        check!(quadratic_add_quadratic(@mut self, @rhs))
    }
}

impl Mul<f64> for Quadratic {
    type Output = Result<Self>;

    fn mul(mut self, rhs: f64) -> Self::Output {
        check!(quadratic_mul_bias(@mut self, rhs))?;
        Ok(self)
    }
}

impl Neg for Quadratic {
    type Output = Result<Self>;

    fn neg(self) -> Self::Output {
        out!(quadratic_neg(@self))
    }
}

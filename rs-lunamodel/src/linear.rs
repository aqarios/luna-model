//! Owning wrapper over a LunaModel linear handle.
//!
//! Mirrors the Rust `Linear`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Linear` still return `Result` here.

use std::ops::{Deref, Mul, Neg};

use lunamodel_cext_vtable::LmLinear;
use rs_lunamodel_derive::RawHandle;

use crate::{
    Result, TryClone, TryIndex,
    api::api,
    traits::{TryAddAssign, TryMulAssign},
};

/// Owning handle to a LunaModel linear (frees on drop).
#[derive(RawHandle)]
pub struct Linear {
    handle: *mut LmLinear,
}

/// Editable snapshot returned by [`Linear::iter_mut`].
#[must_use = "mutations are committed when the editor is dropped; call commit() to observe errors"]
pub struct LinearEditor<'a> {
    target: &'a mut Linear,
    items: Vec<(u32, f64)>,
    committed: bool,
}

impl LinearEditor<'_> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut f64)> {
        self.items.iter_mut().map(linear_item_mut)
    }

    pub fn commit(mut self) -> Result<()> {
        self.commit_inner()?;
        self.committed = true;
        Ok(())
    }

    fn commit_inner(&mut self) -> Result<()> {
        for &(index, bias) in &self.items {
            self.target.set_bias(index, bias)?;
        }
        Ok(())
    }
}

fn linear_item_mut(item: &mut (u32, f64)) -> (u32, &mut f64) {
    (item.0, &mut item.1)
}

impl Deref for LinearEditor<'_> {
    type Target = [(u32, f64)];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<'editor> IntoIterator for &'editor mut LinearEditor<'_> {
    type Item = (u32, &'editor mut f64);
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'editor, (u32, f64)>,
        fn(&'editor mut (u32, f64)) -> (u32, &'editor mut f64),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut().map(linear_item_mut)
    }
}

impl Drop for LinearEditor<'_> {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.commit_inner();
        }
    }
}

impl TryIndex<u32> for Linear {
    type Output = f64;
    fn try_index(&self, index: u32) -> Result<Self::Output> {
        self.index(index)
    }
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

    pub fn for_vars(var_a: (u32, f64), var_b: (u32, f64)) -> Result<Self> {
        out!(linear_for_vars(var_a.0, var_a.1, var_b.0, var_b.1))
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

    pub fn iter(&self) -> Result<std::vec::IntoIter<(u32, f64)>> {
        Ok(self.items()?.into_iter())
    }

    pub fn iter_mut(&mut self) -> Result<LinearEditor<'_>> {
        let items = self.items()?;
        Ok(LinearEditor {
            target: self,
            items,
            committed: false,
        })
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

impl TryAddAssign<Linear> for Linear {
    fn try_add_assign(&mut self, rhs: Linear) -> Result<()> {
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

impl Neg for &Linear {
    type Output = Result<Linear>;
    fn neg(self) -> Self::Output {
        out!(linear_neg(@self))
    }
}

impl std::fmt::Debug for Linear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Linear").field(&self.items().ok()).finish()
    }
}

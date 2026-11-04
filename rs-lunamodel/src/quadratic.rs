//! Owning wrapper over a LunaModel quadratic handle.
//!
//! Mirrors the Rust `Quadratic`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_capi::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `Quadratic` still return `Result` here.

use std::ops::{Deref, Mul, Neg};

use luna_model_ffi_derive::RawHandle;
use lunamodel_capi::LmQuadratic;

use crate::{
    Neighborhood, Result, TryClone, TryIndex,
    api::api,
    traits::{TryAddAssign, TryMulAssign},
};

/// Owning handle to a LunaModel quadratic (frees on drop).
#[derive(RawHandle)]
pub struct Quadratic {
    handle: *mut LmQuadratic,
}

/// Editable grouped snapshot returned by [`Quadratic::iter_mut`].
#[must_use = "mutations are committed when the editor is dropped; call commit() to observe errors"]
pub struct QuadraticEditor<'a> {
    target: &'a mut Quadratic,
    items: Vec<(u32, Neighborhood)>,
    committed: bool,
}

impl QuadraticEditor<'_> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u32, &mut Neighborhood)> {
        self.items.iter_mut().map(quadratic_item_mut)
    }

    pub fn commit(mut self) -> Result<()> {
        self.commit_inner()?;
        self.committed = true;
        Ok(())
    }

    fn commit_inner(&mut self) -> Result<()> {
        let mut replacement = Quadratic::default();
        for (u, neighborhood) in &self.items {
            replacement.push_back_empty(*u)?;
            for (v, bias) in neighborhood.iter() {
                replacement.try_add_assign((*u, v, bias))?;
            }
        }
        std::mem::swap(self.target, &mut replacement);
        Ok(())
    }
}

fn quadratic_item_mut(item: &mut (u32, Neighborhood)) -> (u32, &mut Neighborhood) {
    (item.0, &mut item.1)
}

impl Deref for QuadraticEditor<'_> {
    type Target = [(u32, Neighborhood)];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<'editor> IntoIterator for &'editor mut QuadraticEditor<'_> {
    type Item = (u32, &'editor mut Neighborhood);
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'editor, (u32, Neighborhood)>,
        fn(&'editor mut (u32, Neighborhood)) -> (u32, &'editor mut Neighborhood),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut().map(quadratic_item_mut)
    }
}

impl Drop for QuadraticEditor<'_> {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.commit_inner();
        }
    }
}

impl TryIndex<(u32, u32)> for Quadratic {
    type Output = f64;
    fn try_index(&self, (u, v): (u32, u32)) -> Result<Self::Output> {
        self.index(u, v)
    }
}

impl TryIndex<u32> for Quadratic {
    type Output = Neighborhood;
    fn try_index(&self, index: u32) -> Result<Self::Output> {
        Ok(self
            .iter()?
            .find(|(outer, _)| *outer == index)
            .map(|(_, value)| value)
            .unwrap_or_default())
    }
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

    pub fn iter(&self) -> Result<std::vec::IntoIter<(u32, Neighborhood)>> {
        let mut grouped = std::collections::BTreeMap::<u32, Neighborhood>::new();
        for (u, v, bias) in self.items()? {
            grouped.entry(u).or_default().push_back(v, bias);
        }
        Ok(grouped.into_iter().collect::<Vec<_>>().into_iter())
    }

    pub fn iter_flat(&self) -> Result<std::vec::IntoIter<(u32, u32, f64)>> {
        Ok(self.items()?.into_iter())
    }

    pub fn iter_mut(&mut self) -> Result<QuadraticEditor<'_>> {
        let items = self.iter()?.collect();
        Ok(QuadraticEditor {
            target: self,
            items,
            committed: false,
        })
    }

    pub fn clean(&mut self) -> Result<()> {
        check!(quadratic_clean(@mut self))
    }

    pub fn push_back_empty(&mut self, idx: u32) -> Result<&mut Self> {
        check!(quadratic_push_back_empty(@mut self, idx))?;
        Ok(self)
    }

    pub fn insert_empty(&mut self, pos: usize, idx: u32) -> Result<&mut Self> {
        check!(quadratic_insert_empty(@mut self, pos, idx))?;
        Ok(self)
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

impl TryMulAssign<f64> for Quadratic {
    fn try_mul_assign(&mut self, rhs: f64) -> Result<()> {
        check!(quadratic_mul_bias(@mut self, rhs))
    }
}

impl Neg for Quadratic {
    type Output = Result<Self>;

    fn neg(self) -> Self::Output {
        out!(quadratic_neg(@self))
    }
}

impl Neg for &Quadratic {
    type Output = Result<Quadratic>;
    fn neg(self) -> Self::Output {
        out!(quadratic_neg(@self))
    }
}

impl std::fmt::Debug for Quadratic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Quadratic")
            .field(&self.items().ok())
            .finish()
    }
}

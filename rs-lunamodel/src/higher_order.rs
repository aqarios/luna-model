//! Owning wrapper over a LunaModel higher_order handle.
//!
//! Mirrors the Rust `HigherOrder`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `HigherOrder` still return `Result` here.

use std::ops::{Mul, Neg};

use lunamodel_cext_vtable::LmHigherOrder;
use rs_lunamodel_derive::RawHandle;

use crate::{
    Result, TryClone, TryIndex,
    api::api,
    traits::{TryAddAssign, TryMulAssign},
};

/// Owning handle to a LunaModel higher_order (frees on drop).
#[derive(RawHandle)]
pub struct HigherOrder {
    handle: *mut LmHigherOrder,
}

struct HigherOrderEditItem {
    key: String,
    indices: Vec<u32>,
    bias: f64,
}

/// Editable snapshot returned by [`HigherOrder::iter_mut`].
#[must_use = "mutations are committed when the editor is dropped; call commit() to observe errors"]
pub struct HigherOrderEditor<'a> {
    target: &'a mut HigherOrder,
    items: Vec<HigherOrderEditItem>,
    committed: bool,
}

impl HigherOrderEditor<'_> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut f64)> {
        self.items
            .iter_mut()
            .map(|item| (&item.key, &mut item.bias))
    }

    pub fn commit(mut self) -> Result<()> {
        self.commit_inner()?;
        self.committed = true;
        Ok(())
    }

    fn commit_inner(&mut self) -> Result<()> {
        for item in &self.items {
            self.target.set_bias(&item.indices, item.bias)?;
        }
        Ok(())
    }
}

impl Drop for HigherOrderEditor<'_> {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.commit_inner();
        }
    }
}

impl TryIndex<&[u32]> for HigherOrder {
    type Output = f64;
    fn try_index(&self, indices: &[u32]) -> Result<Self::Output> {
        self.index(indices)
    }
}

impl TryIndex<&String> for HigherOrder {
    type Output = f64;
    fn try_index(&self, key: &String) -> Result<Self::Output> {
        let indices = key
            .split('-')
            .map(|part| {
                part.parse::<u32>().map_err(|error| crate::Error {
                    status: crate::Status::InvalidArgument,
                    message: format!("invalid higher-order key '{key}': {error}"),
                })
            })
            .collect::<Result<Vec<_>>>()?;
        self.index(&indices)
    }
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

    pub fn iter_contrib(&self) -> Result<std::vec::IntoIter<(Vec<u32>, f64)>> {
        Ok(self.items()?.into_iter())
    }

    pub fn iter(&self) -> Result<std::vec::IntoIter<(String, f64)>> {
        Ok(self
            .items()?
            .into_iter()
            .map(|(indices, bias)| {
                (
                    indices
                        .into_iter()
                        .map(|idx| idx.to_string())
                        .collect::<Vec<_>>()
                        .join("-"),
                    bias,
                )
            })
            .collect::<Vec<_>>()
            .into_iter())
    }

    pub fn iter_mut(&mut self) -> Result<HigherOrderEditor<'_>> {
        let items = self
            .items()?
            .into_iter()
            .map(|(indices, bias)| HigherOrderEditItem {
                key: indices
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join("-"),
                indices,
                bias,
            })
            .collect();
        Ok(HigherOrderEditor {
            target: self,
            items,
            committed: false,
        })
    }

    pub fn clean(&mut self) -> Result<()> {
        check!(higher_order_clean(@mut self))
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

impl Neg for &HigherOrder {
    type Output = Result<HigherOrder>;
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

impl TryAddAssign<(Vec<u32>, f64)> for HigherOrder {
    fn try_add_assign(&mut self, rhs: (Vec<u32>, f64)) -> Result<()> {
        self.try_add_assign((rhs.0.as_slice(), rhs.1))
    }
}

impl TryAddAssign<HigherOrder> for HigherOrder {
    fn try_add_assign(&mut self, rhs: HigherOrder) -> Result<()> {
        check!(higher_order_add_higher_order(@mut self, @rhs))
    }
}

impl TryAddAssign<&HigherOrder> for HigherOrder {
    fn try_add_assign(&mut self, rhs: &HigherOrder) -> Result<()> {
        check!(higher_order_add_higher_order(@mut self, @rhs))
    }
}

impl TryMulAssign<f64> for HigherOrder {
    fn try_mul_assign(&mut self, rhs: f64) -> Result<()> {
        check!(higher_order_mul_bias(@mut self, rhs))
    }
}

impl std::fmt::Debug for HigherOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HigherOrder")
            .field(&self.items().ok())
            .finish()
    }
}

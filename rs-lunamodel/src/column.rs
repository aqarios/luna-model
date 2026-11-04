//! Owning wrapper over a host-owned solution column.

use luna_model_ffi_derive::RawHandle;
use lunamodel_capi::{LmColumn, LmVtype};
use num::ToPrimitive;

use crate::{Assignment, Error, Result, Status, TryClone, TryIndex, Vtype, traits::LmTryInto};

/// One variable column within a solution.
///
/// The values remain owned by the LunaModel host. This wrapper intentionally
/// exposes only C-API-backed operations; it does not reproduce the core
/// column's validation or mutation logic locally.
#[derive(RawHandle)]
pub struct Column {
    handle: *mut LmColumn,
}

impl Column {
    pub fn with(assignment: Assignment) -> Result<Self> {
        out!(col_with(~assignment))
    }

    pub fn empty_binary() -> Result<Self> {
        Self::binary(Vec::new())
    }

    pub fn empty_spin() -> Result<Self> {
        Self::spin(Vec::new())
    }

    pub fn empty_integer() -> Result<Self> {
        Self::integer(Vec::new())
    }

    pub fn empty_real() -> Result<Self> {
        Self::real(Vec::new())
    }

    pub fn binary(data: Vec<f64>) -> Result<Self> {
        Self::new(Vtype::Binary, data)
    }

    pub fn spin(data: Vec<f64>) -> Result<Self> {
        Self::new(Vtype::Spin, data)
    }

    pub fn integer(data: Vec<f64>) -> Result<Self> {
        Self::new(Vtype::Integer, data)
    }

    pub fn real(data: Vec<f64>) -> Result<Self> {
        Self::new(Vtype::Real, data)
    }

    fn new(vtype: Vtype, data: Vec<f64>) -> Result<Self> {
        out!(col_new(~vtype, data.as_ptr(), data.len()))
    }

    pub fn len(&self) -> Result<usize> {
        out!(col_len(@self))
    }

    pub fn is_empty(&self) -> Result<bool> {
        out!(col_is_empty(@self))
    }

    pub fn vtype(&self) -> Result<Vtype> {
        let value: LmVtype = out!(col_vtype(@self))?;
        value.lm_try_into()
    }

    pub fn assignment(&self, index: usize) -> Result<Assignment> {
        out!(col_assignment(@self, index))
    }

    pub fn as_assignment(&self, index: usize) -> Result<Assignment> {
        self.assignment(index)
    }

    pub fn as_assignments(&self) -> Result<Vec<Assignment>> {
        (0..self.len()?)
            .map(|index| self.assignment(index))
            .collect()
    }

    pub fn values(&self) -> Result<Vec<f64>> {
        let mut data = std::ptr::null();
        let mut len = 0;
        check!(col_values(@self, &mut data, &mut len))?;
        if data.is_null() && len != 0 {
            return Err(Error {
                status: Status::NullPointer,
                message: "C API returned a null column-values pointer".to_owned(),
                kind_tag: None,
            });
        }
        Ok(if len == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(data, len) }.to_vec()
        })
    }

    pub fn push(&mut self, value: Assignment) -> Result<()> {
        check!(col_push(@mut self, ~value))
    }

    pub fn try_push<N: ToPrimitive>(&mut self, value: N, tol: Option<f64>) -> Result<()> {
        let value = value.to_f64().ok_or_else(|| Error {
            status: Status::InvalidArgument,
            message: "sample value is not representable as f64".to_owned(),
            kind_tag: None,
        })?;
        check!(col_try_push(@mut self, value, ?tol))
    }

    pub fn remove(&mut self, index: usize) -> Result<()> {
        check!(col_remove(@mut self, index))
    }

    pub fn extract(&self, row: usize) -> Result<Self> {
        out!(col_extract(@self, row))
    }

    pub fn filter_by_mask(&self, mask: &[bool]) -> Result<Self> {
        out!(col_filter_by_mask(@self, mask.as_ptr(), mask.len()))
    }
}

impl TryClone for Column {
    fn try_clone(&self) -> Result<Self> {
        out!(col_clone(@self))
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        out_or!(col_eq(@self, @other), false)
    }
}

impl Drop for Column {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, col_free)
        }
    }
}

impl TryIndex<usize> for Column {
    type Output = f64;

    fn try_index(&self, index: usize) -> Result<Self::Output> {
        out!(col_index(@self, index))
    }
}

impl std::fmt::Debug for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Column")
            .field("vtype", &self.vtype().ok())
            .field("len", &self.len().ok())
            .finish()
    }
}

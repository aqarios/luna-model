//! Name-keyed sample adapter shared by C evaluation entry points.

use std::{collections::HashMap, ffi::c_char};

use luna_model::{core::TryIndex, error::LunaModelError};

use crate::{
    error::CapiError,
    utils::{slice_from_c, str_from_c},
};

pub(crate) struct NameSample(HashMap<String, f64>);

impl NameSample {
    /// # Safety
    /// `names` and `values` must each reference `len` readable elements.
    pub(crate) unsafe fn from_c(
        names: *const *const c_char,
        values: *const f64,
        len: usize,
    ) -> Result<Self, CapiError> {
        let names = unsafe { slice_from_c(names, len, "names") }?;
        let values = unsafe { slice_from_c(values, len, "values") }?;
        let mut sample = HashMap::with_capacity(len);
        for (&name, &value) in names.iter().zip(values) {
            sample.insert(unsafe { str_from_c(name, "name") }?.to_owned(), value);
        }
        Ok(Self(sample))
    }
}

impl TryIndex<&str> for NameSample {
    type Output = f64;
    type Err = LunaModelError;

    fn try_index(&self, var: &str) -> Result<&f64, LunaModelError> {
        self.0
            .get(var)
            .ok_or_else(|| LunaModelError::VariableNotExisting(var.into()))
    }
}

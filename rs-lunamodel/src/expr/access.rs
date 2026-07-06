use lunamodel_cext_vtable::{LmVtype, LmVtypeBuffer};
use lunamodel_types::Vtype;

use crate::{
    Env,
    api::api,
    error::{Result, check},
    types::LmFrom,
};

use super::Expression;

impl Expression {
    /// Returns a handle to the shared environment (Rust `expr.env`).
    pub fn env(&self) -> Result<Env> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_env)(self.as_ptr(), &mut out) })?;
        Ok(unsafe { Env::from_raw(out) })
    }

    /// Returns the constant offset (Rust `expr.offset`).
    pub fn offset(&self) -> Result<f64> {
        let mut out = 0.0;
        check(unsafe { (api().expr_offset)(self.as_ptr(), &mut out) })?;
        Ok(out)
    }

    /// Returns the number of distinct variables (Rust `expr.num_vars()`).
    pub fn num_vars(&self) -> Result<usize> {
        let mut out = 0;
        check(unsafe { (api().expr_num_vars)(self.as_ptr(), &mut out) })?;
        Ok(out)
    }

    /// Returns the degree of the expression (Rust `expr.degree()`).
    pub fn degree(&self) -> Result<usize> {
        let mut out = 0;
        check(unsafe { (api().expr_degree)(self.as_ptr(), &mut out) })?;
        Ok(out)
    }

    /// Returns whether the expression is a plain constant
    /// (Rust `expr.is_constant()`).
    pub fn is_constant(&self) -> Result<bool> {
        let mut out = false;
        check(unsafe { (api().expr_is_constant)(self.as_ptr(), &mut out) })?;
        Ok(out)
    }

    /// Returns whether any quadratic term is present (Rust `expr.has_quadratic()`).
    pub fn has_quadratic(&self) -> Result<bool> {
        let mut out = false;
        check(unsafe { (api().expr_has_quadratic)(self.as_ptr(), &mut out) })?;
        Ok(out)
    }

    /// Returns whether any higher-order term is present
    /// (Rust `expr.has_higher_order()`).
    pub fn has_higher_order(&self) -> Result<bool> {
        let mut out = false;
        check(unsafe { (api().expr_has_higher_order)(self.as_ptr(), &mut out) })?;
        Ok(out)
    }

    /// Returns the distinct variable types used by the expression
    /// (Rust `expr.vtypes()`).
    pub fn vtypes(&self) -> Result<Vec<Vtype>> {
        let mut buf = LmVtypeBuffer {
            data: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        };
        check(unsafe { (api().expr_vtypes)(self.as_ptr(), &mut buf) })?;
        let out = if buf.data.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(buf.data, buf.len) }
                .iter()
                .copied()
                .map(<Vtype as LmFrom<LmVtype>>::from)
                .collect()
        };
        unsafe { (api().vtype_buffer_free)(buf) };
        Ok(out)
    }
}

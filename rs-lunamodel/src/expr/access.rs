use lunamodel_cext_vtable::LmVtypeBuffer;
use lunamodel_types::Vtype;

use crate::{Env, error::Result, traits::TryToVec};

use super::Expression;

impl Expression {
    /// Returns a handle to the shared environment (Rust `expr.env`).
    pub fn env(&self) -> Result<Env> {
        out!(expr_env(@self), |out| Ok(unsafe { Env::from_raw(out) }))
    }

    /// Returns the constant offset (Rust `expr.offset`).
    pub fn offset(&self) -> Result<f64> {
        out!(expr_offset(@self))
    }

    /// Returns the number of distinct variables (Rust `expr.num_vars()`).
    pub fn num_vars(&self) -> Result<usize> {
        out!(expr_num_vars(@self))
    }

    /// Returns the degree of the expression (Rust `expr.degree()`).
    pub fn degree(&self) -> Result<usize> {
        out!(expr_degree(@self))
    }

    /// Returns whether the expression is a plain constant
    /// (Rust `expr.is_constant()`).
    pub fn is_constant(&self) -> Result<bool> {
        out!(expr_is_constant(@self))
    }

    /// Returns whether any quadratic term is present (Rust `expr.has_quadratic()`).
    pub fn has_quadratic(&self) -> Result<bool> {
        out!(expr_has_quadratic(@self))
    }

    /// Returns whether any higher-order term is present
    /// (Rust `expr.has_higher_order()`).
    pub fn has_higher_order(&self) -> Result<bool> {
        out!(expr_has_higher_order(@self))
    }

    /// Returns the distinct variable types used by the expression
    /// (Rust `expr.vtypes()`).
    pub fn vtypes(&self) -> Result<Vec<Vtype>> {
        out!(expr_vtypes(@self), |o: LmVtypeBuffer| o.try_to_vec())
    }
}

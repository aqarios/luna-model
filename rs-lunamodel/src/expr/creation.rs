use std::borrow::Borrow;

use crate::{Env, HigherOrder, Linear, Quadratic, VarRef, error::Result};

use super::Expression;

impl Expression {
    /// Native-compatible alias for an empty expression.
    pub fn empty<E>(env: E) -> Result<Self>
    where
        E: Borrow<Env>,
    {
        Self::new(env)
    }

    /// Creates an empty expression bound to `env`.
    pub fn new<E>(env: E) -> Result<Self>
    where
        E: Borrow<Env>,
    {
        let env = env.borrow();
        out!(expr_new(@env))
    }

    /// Creates a constant expression bound to `env`.
    pub fn constant<E>(env: E, value: f64) -> Result<Self>
    where
        E: Borrow<Env>,
    {
        let env = env.borrow();
        out!(expr_new_constant(@env, value))
    }

    /// Creates a constant expression from a float.
    pub fn from_f64(value: f64) -> Result<Self> {
        out!(expr_from_double(value))
    }

    /// Creates a constant expression from an integer.
    pub fn from_usize(value: usize) -> Result<Self> {
        out!(expr_from_size_t(value))
    }

    /// Builds an expression from a linear term collection.
    pub fn from_linear(linear: Linear) -> Result<Self> {
        out!(expr_from_linear(@linear))
    }

    /// Builds an expression from a quadratic term collection.
    pub fn from_quadratic(quadratic: Quadratic) -> Result<Self> {
        out!(expr_from_quadratic(@quadratic))
    }

    /// Builds an expression from a higher-order term collection.
    pub fn from_higher_order(higher_order: HigherOrder) -> Result<Self> {
        out!(expr_from_higher_order(@higher_order))
    }

    /// Fallible equivalent of native `From<VarRef> for Expression`.
    pub fn from_vref(var: &VarRef) -> Result<Self> {
        out!(vref_add_bias(@var, 0.0))
    }

    pub fn from_dense_quadratic(
        dense: &[f64],
        num_vars: usize,
        offset: Option<f64>,
        env: impl Borrow<Env>,
    ) -> Result<Self> {
        let env = env.borrow();
        out!(expr_from_dense_quadratic(
            dense.as_ptr(),
            dense.len(),
            num_vars,
            ?offset,
            @env
        ))
    }
}

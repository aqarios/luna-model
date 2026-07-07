use crate::{Env, HigherOrder, Linear, Quadratic, error::Result};

use super::Expression;

impl Expression {
    /// Creates an empty expression bound to `env`.
    pub fn new(env: &Env) -> Result<Self> {
        out!(expr_new(@env), |out| Ok(Self::from_raw(out)))
    }

    /// Creates a constant expression bound to `env`.
    pub fn constant(env: &Env, value: f64) -> Result<Self> {
        out!(expr_new_constant(@env, value), |out| Ok(Self::from_raw(
            out
        )))
    }

    /// Creates a constant expression from a float.
    pub fn from_f64(value: f64) -> Result<Self> {
        out!(expr_from_double(value), |out| Ok(Self::from_raw(out)))
    }

    /// Creates a constant expression from an integer.
    pub fn from_usize(value: usize) -> Result<Self> {
        out!(expr_from_size_t(value), |out| Ok(Self::from_raw(out)))
    }

    /// Builds an expression from a linear term collection.
    pub fn from_linear(linear: Linear) -> Result<Self> {
        out!(expr_from_linear(@linear), |o| Ok(Self::from_raw(o)))
    }

    /// Builds an expression from a quadratic term collection.
    pub fn from_quadratic(quadratic: Quadratic) -> Result<Self> {
        out!(expr_from_quadratic(@quadratic), |o| Ok(Self::from_raw(o)))
    }

    /// Builds an expression from a higher-order term collection.
    pub fn from_higher_order(higher_order: HigherOrder) -> Result<Self> {
        out!(expr_from_higher_order(@higher_order), |o| Ok(Self::from_raw(o)))
    }
}

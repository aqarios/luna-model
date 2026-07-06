use crate::{Env, error::Result};

use super::Expression;

impl Expression {
    /// Creates an empty expression bound to `env` (Rust `Expression::new(env)`).
    pub fn new(env: &Env) -> Result<Self> {
        out!(expr_new(@env), |out| Ok(Self::from_raw(out)))
    }

    /// Creates a constant expression bound to `env`
    /// (Rust `Expression::constant(env, value)`).
    pub fn constant(env: &Env, value: f64) -> Result<Self> {
        out!(expr_new_constant(@env, value), |out| Ok(Self::from_raw(
            out
        )))
    }

    /// Creates a constant expression from a float (Rust `Expression::from(f64)`).
    pub fn from_f64(value: f64) -> Result<Self> {
        out!(expr_from_double(value), |out| Ok(Self::from_raw(out)))
    }

    /// Creates a constant expression from an integer
    /// (Rust `Expression::from(usize)`).
    pub fn from_usize(value: usize) -> Result<Self> {
        out!(expr_from_size_t(value), |out| Ok(Self::from_raw(out)))
    }
}

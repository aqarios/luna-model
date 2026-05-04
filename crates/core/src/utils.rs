//! Small utility helpers shared across the core crate.

use std::ops::Mul;

use crate::prelude::{Expression, VarRef};

impl From<VarRef> for Expression {
    /// Promotes a variable reference into the equivalent one-term expression.
    ///
    /// This goes through the regular multiplication path instead of manually
    /// constructing linear storage so that all environment-sensitive checks stay
    /// centralized in one place.
    fn from(var: VarRef) -> Self {
        Expression::constant(var.env.clone(), 1.0)
            .mul(&var)
            .expect("the environment changed during cloning")
    }
}

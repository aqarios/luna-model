use std::ops::Mul;

use crate::prelude::{Expression, VarRef};

impl From<VarRef> for Expression {
    fn from(var: VarRef) -> Self {
        Expression::constant(var.env.clone(), 1.0)
            .mul(&var)
            .expect("the environment changed during cloning")
    }
}

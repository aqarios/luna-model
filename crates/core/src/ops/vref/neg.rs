use std::ops::Neg;

use crate::{Expression, ops::LmSubAssign, prelude::VarRef, traits::Editable};

impl Neg for &VarRef {
    type Output = Expression;

    fn neg(self) -> Self::Output {
        Expression::empty(self.env.clone()).edit(|e| {
            e.sub_assign(self)
                .expect("variable.neg() should work without errors.")
        })
    }
}

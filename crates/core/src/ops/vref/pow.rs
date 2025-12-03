use crate::{
    Expression,
    ops::{LmAddAssign, LmPow, LmPowAssign},
    prelude::VarRef,
    traits::Editable,
};
use lunamodel_error::LunaModelResult;

impl LmPow for &VarRef {
    type Output = Expression;
    fn pow(self, sup: usize) -> LunaModelResult<Self::Output> {
        let mut expr = Expression::empty(self.env.clone()).maybe_edit(|e| e.add_assign(self))?;
        expr.pow_assign(sup)?;
        Ok(expr)
    }
}

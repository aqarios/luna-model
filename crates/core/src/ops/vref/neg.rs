use std::ops::Neg;

use lunamodel_error::LunaModelResult;

use crate::{Expression, ops::LmSubAssign, prelude::VarRef, traits::Editable};

impl Neg for &VarRef {
    type Output = LunaModelResult<Expression>;

    fn neg(self) -> Self::Output {
        self.check_living()?;
        Expression::empty(self.env.clone()).maybe_edit(|e| e.sub_assign(self))
    }
}

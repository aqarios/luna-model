use std::ops::Neg;

use lunamodel_error::LunaModelResult;

use crate::{Expression, ops::LmSubAssign, prelude::VarRef, traits::Editable};

impl Neg for &VarRef {
    type Output = LunaModelResult<Expression>;

    /// Builds the expression `-self`.
    fn neg(self) -> Self::Output {
        self.check_living()?;
        Expression::empty(self.env.clone()).maybe_edit(|e| e.sub_assign(self))
    }
}

impl Neg for VarRef {
    type Output = LunaModelResult<Expression>;

    /// Owned forwarding overload for unary negation.
    fn neg(self) -> Self::Output {
        -&self
    }
}

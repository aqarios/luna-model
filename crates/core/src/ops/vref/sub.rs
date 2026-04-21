use std::ops::Sub;

use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::{
    Expression,
    ops::{LmAddAssign, LmSubAssign, utils::check_envs},
    prelude::VarRef,
    traits::Editable,
};

impl Sub<Self> for &VarRef {
    type Output = LunaModelResult<Expression>;
    fn sub(self, rhs: Self) -> Self::Output {
        check_envs(self, rhs)?;
        self.check_living()?;
        rhs.check_living()?;
        let mut out = Expression::empty(self.env.clone()).maybe_edit(|e| e.add_assign(self))?;
        out.sub_assign(rhs)?;
        Ok(out)
    }
}

impl Sub<Self> for VarRef {
    type Output = LunaModelResult<Expression>;
    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl Sub<Bias> for &VarRef {
    type Output = LunaModelResult<Expression>;
    fn sub(self, rhs: Bias) -> Self::Output {
        self.check_living()?;
        let mut out = Expression::empty(self.env.clone()).maybe_edit(|e| e.add_assign(self))?;
        out.sub_assign(rhs)?;
        Ok(out)
    }
}

impl Sub<Bias> for VarRef {
    type Output = LunaModelResult<Expression>;
    fn sub(self, rhs: Bias) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl Sub<usize> for &VarRef {
    type Output = LunaModelResult<Expression>;
    fn sub(self, rhs: usize) -> Self::Output {
        self.sub(rhs as Bias)
    }
}

impl Sub<usize> for VarRef {
    type Output = LunaModelResult<Expression>;
    fn sub(self, rhs: usize) -> Self::Output {
        self.sub(rhs as Bias)
    }
}

impl Sub<VarRef> for usize {
    type Output = LunaModelResult<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        rhs.sub(self)
    }
}

impl Sub<VarRef> for Bias {
    type Output = LunaModelResult<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        rhs.sub(self)
    }
}

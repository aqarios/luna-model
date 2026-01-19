use std::ops::Add;

use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::{
    Expression,
    ops::{LmAddAssign, utils::check_envs},
    prelude::VarRef,
    traits::Editable,
};

impl Add<Self> for &VarRef {
    type Output = LunaModelResult<Expression>;
    fn add(self, rhs: Self) -> Self::Output {
        check_envs(self, rhs)?;
        self.check_living()?;
        rhs.check_living()?;
        let mut out = Expression::empty(self.env.clone()).maybe_edit(|e| e.add_assign(self))?;
        out.add_assign(rhs)?;
        Ok(out)
    }
}

impl Add<Bias> for &VarRef {
    type Output = LunaModelResult<Expression>;
    fn add(self, rhs: Bias) -> Self::Output {
        self.check_living()?;
        let mut out = Expression::empty(self.env.clone()).maybe_edit(|e| e.add_assign(self))?;
        out.add_assign(rhs)?;
        Ok(out)
    }
}

impl Add<usize> for &VarRef {
    type Output = LunaModelResult<Expression>;
    fn add(self, rhs: usize) -> Self::Output {
        self.add(rhs as Bias)
    }
}

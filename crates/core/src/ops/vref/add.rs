//! Addition implementations for variable references.

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

    /// Builds the linear expression `self + rhs`.
    ///
    /// Both variable references must be live and belong to the same
    /// environment.
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

    /// Builds the affine expression `self + rhs`.
    fn add(self, rhs: Bias) -> Self::Output {
        self.check_living()?;
        let mut out = Expression::empty(self.env.clone()).maybe_edit(|e| e.add_assign(self))?;
        out.add_assign(rhs)?;
        Ok(out)
    }
}

impl Add<usize> for &VarRef {
    type Output = LunaModelResult<Expression>;

    /// Convenience overload forwarding integer literals through `Bias`.
    fn add(self, rhs: usize) -> Self::Output {
        self.add(rhs as Bias)
    }
}

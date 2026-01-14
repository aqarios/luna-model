use crate::{
    Expression,
    ops::{LmAddAssign, LmMulAssign, LmPow},
    prelude::VarRef,
};
use lunamodel_error::LunaModelResult;

impl LmPow for &VarRef {
    type Output = Expression;
    fn pow(self, sup: usize) -> LunaModelResult<Self::Output> {
        self.check_living()?;
        let mut base = Expression::empty(self.env.clone());
        match sup {
            0 => base.offset = 1.0,
            1 => base.add_assign(self)?,
            s => {
                base.add_assign(self)?;
                for _ in 0..(s - 1) {
                    base.mul_assign(self)?;
                }
            }
        };
        Ok(base)
    }
}

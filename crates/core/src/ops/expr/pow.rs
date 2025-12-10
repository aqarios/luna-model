use crate::{
    Expression,
    ops::{LmMulAssign, LmPow, LmPowAssign},
    traits::Editable,
};
use lunamodel_error::LunaModelResult;

impl LmPowAssign for Expression {
    fn pow_assign(&mut self, exp: usize) -> LunaModelResult<()> {
        match exp {
            0 => *self = Expression::empty(self.env.clone()).edit(|e| e.offset = 1.0),
            1 => (),
            s => {
                let other = self.clone();
                for _ in 0..(s - 1) {
                    self.mul_assign(&other)?;
                }
            }
        };
        Ok(())
    }
}

impl LmPow for &Expression {
    type Output = Expression;
    fn pow(self, sup: usize) -> LunaModelResult<Self::Output> {
        let mut slf = self.clone();
        slf.pow_assign(sup)?;
        Ok(slf)
    }
}

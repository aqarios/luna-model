use crate::{
    Expression,
    ops::{LmMulAssign, LmPow, LmPowAssign},
    traits::Editable,
};
use lunamodel_error::LunaModelResult;

impl LmPowAssign for Expression {
    /// Raises the expression to a non-negative integer power.
    ///
    /// This currently uses repeated multiplication, which is simple but not
    /// asymptotically optimal for large exponents.
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

    /// Returns a powered clone of the expression.
    fn pow(self, sup: usize) -> LunaModelResult<Self::Output> {
        let mut slf = self.clone();
        slf.pow_assign(sup)?;
        Ok(slf)
    }
}

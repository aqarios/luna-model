use crate::{Expression, LmPow, LmPowAssign, Result};

impl Expression {
    /// `self.pow(n)` (Rust core expression exponentiation).
    pub fn pow(&self, n: usize) -> Result<Self> {
        out!(expr_pow(@self, n))
    }

    /// `self = self.pow(n)`, in place.
    pub fn pow_assign(&mut self, n: usize) -> Result<()> {
        check!(expr_ipow(@mut self, n))
    }
}

impl LmPow for Expression {
    type Output = Expression;
    fn pow(self, exp: usize) -> Result<Self::Output> {
        Expression::pow(&self, exp)
    }
}

impl LmPow for &Expression {
    type Output = Expression;
    fn pow(self, exp: usize) -> Result<Self::Output> {
        Expression::pow(self, exp)
    }
}

impl LmPowAssign for Expression {
    fn pow_assign(&mut self, exp: usize) -> Result<()> {
        Expression::pow_assign(self, exp)
    }
}

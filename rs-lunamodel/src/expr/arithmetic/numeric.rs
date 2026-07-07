use std::ops::{Add, Mul, Sub};

use crate::{
    Expression, Result,
    traits::{TryAddAssign, TryMulAssign, TrySubAssign},
};

impl TryAddAssign<f64> for Expression {
    fn try_add_assign(&mut self, rhs: f64) -> Result<()> {
        check!(expr_iadd_bias(@mut self, rhs))
    }
}

impl Add<f64> for Expression {
    type Output = Result<Self>;

    fn add(self, rhs: f64) -> Self::Output {
        let mut out = self;
        out.try_add_assign(rhs)?;
        Ok(out)
    }
}

impl Add<f64> for &Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: f64) -> Self::Output {
        out!(expr_add_bias(@self, rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl TryAddAssign<usize> for Expression {
    fn try_add_assign(&mut self, rhs: usize) -> Result<()> {
        self.try_add_assign(rhs as f64)
    }
}

impl Add<usize> for Expression {
    type Output = Result<Self>;

    fn add(self, rhs: usize) -> Self::Output {
        self + rhs as f64
    }
}

impl Add<usize> for &Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: usize) -> Self::Output {
        self + rhs as f64
    }
}

impl Add<Expression> for f64 {
    type Output = Result<Expression>;

    fn add(self, rhs: Expression) -> Self::Output {
        rhs + self
    }
}

impl Add<&Expression> for f64 {
    type Output = Result<Expression>;

    fn add(self, rhs: &Expression) -> Self::Output {
        rhs + self
    }
}

impl Add<Expression> for usize {
    type Output = Result<Expression>;

    fn add(self, rhs: Expression) -> Self::Output {
        rhs + self
    }
}

impl Add<&Expression> for usize {
    type Output = Result<Expression>;

    fn add(self, rhs: &Expression) -> Self::Output {
        rhs + self
    }
}

impl TrySubAssign<f64> for Expression {
    fn try_sub_assign(&mut self, rhs: f64) -> Result<()> {
        check!(expr_isub_bias(@mut self, rhs))
    }
}

impl Sub<f64> for Expression {
    type Output = Result<Self>;

    fn sub(self, rhs: f64) -> Self::Output {
        let mut out = self;
        out.try_sub_assign(rhs)?;
        Ok(out)
    }
}

impl Sub<f64> for &Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: f64) -> Self::Output {
        out!(expr_sub_bias(@self, rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl TrySubAssign<usize> for Expression {
    fn try_sub_assign(&mut self, rhs: usize) -> Result<()> {
        self.try_sub_assign(rhs as f64)
    }
}

impl Sub<usize> for Expression {
    type Output = Result<Self>;

    fn sub(self, rhs: usize) -> Self::Output {
        self - rhs as f64
    }
}

impl Sub<usize> for &Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: usize) -> Self::Output {
        self - rhs as f64
    }
}

impl Sub<Expression> for f64 {
    type Output = Result<Expression>;

    fn sub(self, rhs: Expression) -> Self::Output {
        out!(expr_rsub_bias(@rhs, self), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<&Expression> for f64 {
    type Output = Result<Expression>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        out!(expr_rsub_bias(@rhs, self), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<Expression> for usize {
    type Output = Result<Expression>;

    fn sub(self, rhs: Expression) -> Self::Output {
        self as f64 - rhs
    }
}

impl Sub<&Expression> for usize {
    type Output = Result<Expression>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        self as f64 - rhs
    }
}

impl TryMulAssign<f64> for Expression {
    fn try_mul_assign(&mut self, rhs: f64) -> Result<()> {
        check!(expr_imul_bias(@mut self, rhs))
    }
}

impl Mul<f64> for Expression {
    type Output = Result<Self>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut out = self;
        out.try_mul_assign(rhs)?;
        Ok(out)
    }
}

impl Mul<f64> for &Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: f64) -> Self::Output {
        out!(expr_mul_bias(@self, rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl TryMulAssign<usize> for Expression {
    fn try_mul_assign(&mut self, rhs: usize) -> Result<()> {
        self.try_mul_assign(rhs as f64)
    }
}

impl Mul<usize> for Expression {
    type Output = Result<Self>;

    fn mul(self, rhs: usize) -> Self::Output {
        self * rhs as f64
    }
}

impl Mul<usize> for &Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: usize) -> Self::Output {
        self * rhs as f64
    }
}

impl Mul<Expression> for f64 {
    type Output = Result<Expression>;

    fn mul(self, rhs: Expression) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Expression> for f64 {
    type Output = Result<Expression>;

    fn mul(self, rhs: &Expression) -> Self::Output {
        rhs * self
    }
}

impl Mul<Expression> for usize {
    type Output = Result<Expression>;

    fn mul(self, rhs: Expression) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Expression> for usize {
    type Output = Result<Expression>;

    fn mul(self, rhs: &Expression) -> Self::Output {
        rhs * self
    }
}

use std::ops::{Add, Mul, Sub};

use crate::{
    Expression, Result,
    api::api,
    error::check,
    traits::{TryAddAssign, TryMulAssign, TrySubAssign},
};

impl TryAddAssign<f64> for Expression {
    fn try_add_assign(&mut self, rhs: f64) -> Result<()> {
        check(unsafe { (api().expr_iadd_bias)(self.handle, rhs) })
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
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_add_bias)(self.as_ptr(), rhs, &mut out) })?;
        Ok(Expression::from_raw(out))
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
        check(unsafe { (api().expr_isub_bias)(self.handle, rhs) })
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
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_sub_bias)(self.as_ptr(), rhs, &mut out) })?;
        Ok(Expression::from_raw(out))
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
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_rsub_bias)(rhs.as_ptr(), self, &mut out) })?;
        Ok(Expression::from_raw(out))
    }
}

impl Sub<&Expression> for f64 {
    type Output = Result<Expression>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_rsub_bias)(rhs.as_ptr(), self, &mut out) })?;
        Ok(Expression::from_raw(out))
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
        check(unsafe { (api().expr_imul_bias)(self.handle, rhs) })
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
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_mul_bias)(self.as_ptr(), rhs, &mut out) })?;
        Ok(Expression::from_raw(out))
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

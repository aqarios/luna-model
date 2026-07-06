use std::ops::{Add, Mul, Neg, Sub};

use crate::{
    Expression, Result,
    api::api,
    error::check,
    traits::{TryAddAssign, TryMulAssign, TrySubAssign},
};

impl TryAddAssign<Expression> for Expression {
    fn try_add_assign(&mut self, rhs: Expression) -> Result<()> {
        self.try_add_assign(&rhs)
    }
}

impl TryAddAssign<&Expression> for Expression {
    fn try_add_assign(&mut self, rhs: &Expression) -> Result<()> {
        check(unsafe { (api().expr_iadd_expr)(self.handle, rhs.as_ptr()) })
    }
}

impl Add<Expression> for Expression {
    type Output = Result<Self>;

    fn add(self, rhs: Expression) -> Self::Output {
        let mut out = self;
        out.try_add_assign(&rhs)?;
        Ok(out)
    }
}

impl Add<&Expression> for Expression {
    type Output = Result<Self>;

    fn add(self, rhs: &Expression) -> Self::Output {
        let mut out = self;
        out.try_add_assign(rhs)?;
        Ok(out)
    }
}

impl Add<Expression> for &Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: Expression) -> Self::Output {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_add_expr)(self.as_ptr(), rhs.as_ptr(), &mut out) })?;
        Ok(Expression::from_raw(out))
    }
}

impl Add<&Expression> for &Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: &Expression) -> Self::Output {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_add_expr)(self.as_ptr(), rhs.as_ptr(), &mut out) })?;
        Ok(Expression::from_raw(out))
    }
}

impl Add<Expression> for Result<Expression> {
    type Output = Result<Expression>;

    fn add(self, rhs: Expression) -> Self::Output {
        self.and_then(|lhs| lhs + rhs)
    }
}

impl Add<&Expression> for Result<Expression> {
    type Output = Result<Expression>;

    fn add(self, rhs: &Expression) -> Self::Output {
        self.and_then(|lhs| lhs + rhs)
    }
}

impl Add<Result<Expression>> for Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: Result<Expression>) -> Self::Output {
        rhs.and_then(|rhs| self + rhs)
    }
}

impl Add<Result<Expression>> for &Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: Result<Expression>) -> Self::Output {
        rhs.and_then(|rhs| self + rhs)
    }
}

impl TrySubAssign<Expression> for Expression {
    fn try_sub_assign(&mut self, rhs: Expression) -> Result<()> {
        self.try_sub_assign(&rhs)
    }
}

impl TrySubAssign<&Expression> for Expression {
    fn try_sub_assign(&mut self, rhs: &Expression) -> Result<()> {
        check(unsafe { (api().expr_isub_expr)(self.handle, rhs.as_ptr()) })
    }
}

impl Sub<Expression> for Expression {
    type Output = Result<Self>;

    fn sub(self, rhs: Expression) -> Self::Output {
        let mut out = self;
        out.try_sub_assign(&rhs)?;
        Ok(out)
    }
}

impl Sub<&Expression> for Expression {
    type Output = Result<Self>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        let mut out = self;
        out.try_sub_assign(rhs)?;
        Ok(out)
    }
}

impl Sub<Expression> for &Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: Expression) -> Self::Output {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_sub_expr)(self.as_ptr(), rhs.as_ptr(), &mut out) })?;
        Ok(Expression::from_raw(out))
    }
}

impl Sub<&Expression> for &Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_sub_expr)(self.as_ptr(), rhs.as_ptr(), &mut out) })?;
        Ok(Expression::from_raw(out))
    }
}

impl Sub<Expression> for Result<Expression> {
    type Output = Result<Expression>;

    fn sub(self, rhs: Expression) -> Self::Output {
        self.and_then(|lhs| lhs - rhs)
    }
}

impl Sub<&Expression> for Result<Expression> {
    type Output = Result<Expression>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        self.and_then(|lhs| lhs - rhs)
    }
}

impl Sub<Result<Expression>> for Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: Result<Expression>) -> Self::Output {
        rhs.and_then(|rhs| self - rhs)
    }
}

impl Sub<Result<Expression>> for &Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: Result<Expression>) -> Self::Output {
        rhs.and_then(|rhs| self - rhs)
    }
}

impl TryMulAssign<Expression> for Expression {
    fn try_mul_assign(&mut self, rhs: Expression) -> Result<()> {
        self.try_mul_assign(&rhs)
    }
}

impl TryMulAssign<&Expression> for Expression {
    fn try_mul_assign(&mut self, rhs: &Expression) -> Result<()> {
        check(unsafe { (api().expr_imul_expr)(self.handle, rhs.as_ptr()) })
    }
}

impl Mul<Expression> for Expression {
    type Output = Result<Self>;

    fn mul(self, rhs: Expression) -> Self::Output {
        let mut out = self;
        out.try_mul_assign(&rhs)?;
        Ok(out)
    }
}

impl Mul<&Expression> for Expression {
    type Output = Result<Self>;

    fn mul(self, rhs: &Expression) -> Self::Output {
        let mut out = self;
        out.try_mul_assign(rhs)?;
        Ok(out)
    }
}

impl Mul<Expression> for &Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: Expression) -> Self::Output {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_mul_expr)(self.as_ptr(), rhs.as_ptr(), &mut out) })?;
        Ok(Expression::from_raw(out))
    }
}

impl Mul<&Expression> for &Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: &Expression) -> Self::Output {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_mul_expr)(self.as_ptr(), rhs.as_ptr(), &mut out) })?;
        Ok(Expression::from_raw(out))
    }
}

impl Mul<Expression> for Result<Expression> {
    type Output = Result<Expression>;

    fn mul(self, rhs: Expression) -> Self::Output {
        self.and_then(|lhs| lhs * rhs)
    }
}

impl Mul<&Expression> for Result<Expression> {
    type Output = Result<Expression>;

    fn mul(self, rhs: &Expression) -> Self::Output {
        self.and_then(|lhs| lhs * rhs)
    }
}

impl Mul<Result<Expression>> for Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: Result<Expression>) -> Self::Output {
        rhs.and_then(|rhs| self * rhs)
    }
}

impl Mul<Result<Expression>> for &Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: Result<Expression>) -> Self::Output {
        rhs.and_then(|rhs| self * rhs)
    }
}

impl Neg for Expression {
    type Output = Result<Self>;

    fn neg(self) -> Self::Output {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_neg)(self.as_ptr(), &mut out) })?;
        Ok(Expression::from_raw(out))
    }
}

impl Neg for &Expression {
    type Output = Result<Expression>;

    fn neg(self) -> Self::Output {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_neg)(self.as_ptr(), &mut out) })?;
        Ok(Expression::from_raw(out))
    }
}

use std::ops::{Add, Mul, Neg, Sub};

use crate::{
    api::api,
    error::{Result, check},
};

use super::Expression;

impl Expression {
    /// `self + bias` (Rust `&expr + bias`).
    fn add_bias(&self, bias: f64) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_add_bias)(self.as_ptr(), bias, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self + other` (Rust `&expr + &expr`).
    fn add(&self, other: &Self) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_add_expr)(self.as_ptr(), other.as_ptr(), &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self - bias` (Rust `&expr - bias`).
    fn sub_bias(&self, bias: f64) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_sub_bias)(self.as_ptr(), bias, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `bias - self` (Rust `bias - &expr`).
    fn rsub_bias(&self, bias: f64) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_rsub_bias)(self.as_ptr(), bias, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self - other` (Rust `&expr - &expr`).
    fn sub(&self, other: &Self) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_sub_expr)(self.as_ptr(), other.as_ptr(), &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self * bias` (Rust `&expr * bias`).
    fn mul_bias(&self, bias: f64) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_mul_bias)(self.as_ptr(), bias, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self * other` (Rust `&expr * &expr`).
    fn mul(&self, other: &Self) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_mul_expr)(self.as_ptr(), other.as_ptr(), &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `-self` (Rust `-&expr`).
    fn neg(&self) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_neg)(self.as_ptr(), &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self.pow(n)` (Rust `expr.pow(n)`).
    pub fn pow(&self, n: usize) -> Result<Self> {
        let mut out = std::ptr::null_mut();
        check(unsafe { (api().expr_pow)(self.as_ptr(), n, &mut out) })?;
        Ok(Self::from_raw(out))
    }

    /// `self += bias` (fallible in-place addition).
    fn add_assign_bias(&mut self, bias: f64) -> Result<()> {
        check(unsafe { (api().expr_iadd_bias)(self.handle, bias) })
    }

    /// `self += other` (fallible in-place addition).
    fn add_assign(&mut self, other: &Self) -> Result<()> {
        check(unsafe { (api().expr_iadd_expr)(self.handle, other.as_ptr()) })
    }

    /// `self -= bias` (fallible in-place subtraction).
    fn sub_assign_bias(&mut self, bias: f64) -> Result<()> {
        check(unsafe { (api().expr_isub_bias)(self.handle, bias) })
    }

    /// `self -= other` (fallible in-place subtraction).
    fn sub_assign(&mut self, other: &Self) -> Result<()> {
        check(unsafe { (api().expr_isub_expr)(self.handle, other.as_ptr()) })
    }

    /// `self *= bias` (fallible in-place multiplication).
    fn mul_assign_bias(&mut self, bias: f64) -> Result<()> {
        check(unsafe { (api().expr_imul_bias)(self.handle, bias) })
    }

    /// `self *= other` (fallible in-place multiplication).
    fn mul_assign(&mut self, other: &Self) -> Result<()> {
        check(unsafe { (api().expr_imul_expr)(self.handle, other.as_ptr()) })
    }

    /// `self = self.pow(n)`, in place.
    pub fn pow_assign(&mut self, n: usize) -> Result<()> {
        check(unsafe { (api().expr_ipow)(self.handle, n) })
    }
}

impl Add<f64> for Expression {
    type Output = Result<Self>;

    fn add(mut self, rhs: f64) -> Self::Output {
        self.add_assign_bias(rhs)?;
        Ok(self)
    }
}

impl Add<f64> for &Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: f64) -> Self::Output {
        self.add_bias(rhs)
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

impl Add<Expression> for Expression {
    type Output = Result<Self>;

    fn add(mut self, rhs: Expression) -> Self::Output {
        Expression::add_assign(&mut self, &rhs)?;
        Ok(self)
    }
}

impl Add<&Expression> for Expression {
    type Output = Result<Self>;

    fn add(mut self, rhs: &Expression) -> Self::Output {
        Expression::add_assign(&mut self, rhs)?;
        Ok(self)
    }
}

impl Add<Expression> for &Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: Expression) -> Self::Output {
        Expression::add(self, &rhs)
    }
}

impl Add<&Expression> for &Expression {
    type Output = Result<Expression>;

    fn add(self, rhs: &Expression) -> Self::Output {
        Expression::add(self, rhs)
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

impl Sub<f64> for Expression {
    type Output = Result<Self>;

    fn sub(mut self, rhs: f64) -> Self::Output {
        self.sub_assign_bias(rhs)?;
        Ok(self)
    }
}

impl Sub<f64> for &Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: f64) -> Self::Output {
        self.sub_bias(rhs)
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

impl Sub<Expression> for Expression {
    type Output = Result<Self>;

    fn sub(mut self, rhs: Expression) -> Self::Output {
        Expression::sub_assign(&mut self, &rhs)?;
        Ok(self)
    }
}

impl Sub<&Expression> for Expression {
    type Output = Result<Self>;

    fn sub(mut self, rhs: &Expression) -> Self::Output {
        Expression::sub_assign(&mut self, rhs)?;
        Ok(self)
    }
}

impl Sub<Expression> for &Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: Expression) -> Self::Output {
        Expression::sub(self, &rhs)
    }
}

impl Sub<&Expression> for &Expression {
    type Output = Result<Expression>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        Expression::sub(self, rhs)
    }
}

impl Sub<Expression> for f64 {
    type Output = Result<Expression>;

    fn sub(self, rhs: Expression) -> Self::Output {
        rhs.rsub_bias(self)
    }
}

impl Sub<&Expression> for f64 {
    type Output = Result<Expression>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        rhs.rsub_bias(self)
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

impl Mul<f64> for Expression {
    type Output = Result<Self>;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.mul_assign_bias(rhs)?;
        Ok(self)
    }
}

impl Mul<f64> for &Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: f64) -> Self::Output {
        self.mul_bias(rhs)
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

impl Mul<Expression> for Expression {
    type Output = Result<Self>;

    fn mul(mut self, rhs: Expression) -> Self::Output {
        Expression::mul_assign(&mut self, &rhs)?;
        Ok(self)
    }
}

impl Mul<&Expression> for Expression {
    type Output = Result<Self>;

    fn mul(mut self, rhs: &Expression) -> Self::Output {
        Expression::mul_assign(&mut self, rhs)?;
        Ok(self)
    }
}

impl Mul<Expression> for &Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: Expression) -> Self::Output {
        Expression::mul(self, &rhs)
    }
}

impl Mul<&Expression> for &Expression {
    type Output = Result<Expression>;

    fn mul(self, rhs: &Expression) -> Self::Output {
        Expression::mul(self, rhs)
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
        Expression::neg(&self)
    }
}

impl Neg for &Expression {
    type Output = Result<Expression>;

    fn neg(self) -> Self::Output {
        Expression::neg(self)
    }
}

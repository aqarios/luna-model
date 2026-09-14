use std::ops::{Add, Mul, Sub};
use std::ops::{Neg, Not};

use super::VarRef;
use crate::Expression;
use crate::LmPow;

use crate::Result;

impl VarRef {
    /// Raises the variable to the `n`-th power.
    pub fn pow(&self, n: usize) -> Result<Expression> {
        out!(vref_pow(@self, n))
    }

    /// Evaluates whether `value` satisfies the variable's bounds, within an
    /// optional tolerance. This is a metadata check against the variable's bound
    /// interval and does not inspect any expression context.
    pub fn evaluate(&self, value: f64, tol: Option<f64>) -> Result<bool> {
        out!(vref_eval(@self, value, ?tol))
    }
}

impl LmPow for VarRef {
    type Output = Expression;
    fn pow(self, exp: usize) -> Result<Self::Output> {
        VarRef::pow(&self, exp)
    }
}

impl LmPow for &VarRef {
    type Output = Expression;
    fn pow(self, exp: usize) -> Result<Self::Output> {
        VarRef::pow(self, exp)
    }
}

// ADD

impl Add<VarRef> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: VarRef) -> Self::Output {
        out!(vref_add_vref(@self, @rhs))
    }
}

impl Add<&VarRef> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        out!(vref_add_vref(@self, @rhs))
    }
}

impl Add<VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: VarRef) -> Self::Output {
        out!(vref_add_vref(@self, @rhs))
    }
}

impl Add<&VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        out!(vref_add_vref(@self, @rhs))
    }
}

impl Add<f64> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: f64) -> Self::Output {
        out!(vref_add_bias(@self, rhs))
    }
}

impl Add<f64> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: f64) -> Self::Output {
        out!(vref_add_bias(@self, rhs))
    }
}

impl Add<usize> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: usize) -> Self::Output {
        out!(vref_add_bias(@self, rhs as f64))
    }
}

impl Add<usize> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: usize) -> Self::Output {
        out!(vref_add_bias(@self, rhs as f64))
    }
}

/* Reverse scalar addition commutes: `scalar + vref == vref + scalar`. */

impl Add<VarRef> for f64 {
    type Output = Result<Expression>;

    fn add(self, rhs: VarRef) -> Self::Output {
        rhs + self
    }
}

impl Add<&VarRef> for f64 {
    type Output = Result<Expression>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        rhs + self
    }
}

impl Add<VarRef> for usize {
    type Output = Result<Expression>;

    fn add(self, rhs: VarRef) -> Self::Output {
        rhs + self
    }
}

impl Add<&VarRef> for usize {
    type Output = Result<Expression>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        rhs + self
    }
}

// SUB

impl Sub<VarRef> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        out!(vref_sub_vref(@self, @rhs))
    }
}

impl Sub<&VarRef> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        out!(vref_sub_vref(@self, @rhs))
    }
}

impl Sub<VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        out!(vref_sub_vref(@self, @rhs))
    }
}

impl Sub<&VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        out!(vref_sub_vref(@self, @rhs))
    }
}

impl Sub<Expression> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: Expression) -> Self::Output {
        out!(vref_sub_expr(@self, @rhs))
    }
}

impl Sub<&Expression> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        out!(vref_sub_expr(@self, @rhs))
    }
}

impl Sub<Expression> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: Expression) -> Self::Output {
        out!(vref_sub_expr(@self, @rhs))
    }
}

impl Sub<&Expression> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        out!(vref_sub_expr(@self, @rhs))
    }
}

impl Sub<f64> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: f64) -> Self::Output {
        out!(vref_sub_bias(@self, rhs))
    }
}

impl Sub<f64> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: f64) -> Self::Output {
        out!(vref_sub_bias(@self, rhs))
    }
}

impl Sub<usize> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: usize) -> Self::Output {
        out!(vref_sub_bias(@self, rhs as f64))
    }
}

impl Sub<usize> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: usize) -> Self::Output {
        out!(vref_sub_bias(@self, rhs as f64))
    }
}

impl Sub<VarRef> for f64 {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        out!(vref_rsub_bias(@rhs, self))
    }
}

impl Sub<&VarRef> for f64 {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        out!(vref_rsub_bias(@rhs, self))
    }
}

impl Sub<VarRef> for usize {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        out!(vref_rsub_bias(@rhs, self as f64))
    }
}

impl Sub<&VarRef> for usize {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        out!(vref_rsub_bias(@rhs, self as f64))
    }
}

// MUL

impl Mul<VarRef> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: VarRef) -> Self::Output {
        out!(vref_mul_vref(@self, @rhs))
    }
}

impl Mul<&VarRef> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        out!(vref_mul_vref(@self, @rhs))
    }
}

impl Mul<VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: VarRef) -> Self::Output {
        out!(vref_mul_vref(@self, @rhs))
    }
}

impl Mul<&VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        out!(vref_mul_vref(@self, @rhs))
    }
}

impl Mul<f64> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: f64) -> Self::Output {
        out!(vref_mul_bias(@self, rhs))
    }
}

impl Mul<f64> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: f64) -> Self::Output {
        out!(vref_mul_bias(@self, rhs))
    }
}

impl Mul<usize> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: usize) -> Self::Output {
        out!(vref_mul_bias(@self, rhs as f64))
    }
}

impl Mul<usize> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: usize) -> Self::Output {
        out!(vref_mul_bias(@self, rhs as f64))
    }
}

/* Reverse scalar multiplication commutes: `scalar * vref == vref * scalar`. */

impl Mul<VarRef> for f64 {
    type Output = Result<Expression>;

    fn mul(self, rhs: VarRef) -> Self::Output {
        rhs * self
    }
}

impl Mul<&VarRef> for f64 {
    type Output = Result<Expression>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        rhs * self
    }
}

impl Mul<VarRef> for usize {
    type Output = Result<Expression>;

    fn mul(self, rhs: VarRef) -> Self::Output {
        rhs * self
    }
}

impl Mul<&VarRef> for usize {
    type Output = Result<Expression>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        rhs * self
    }
}

impl Neg for VarRef {
    type Output = Result<Expression>;

    fn neg(self) -> Self::Output {
        out!(vref_neg(@self))
    }
}

impl Neg for &VarRef {
    type Output = Result<Expression>;

    fn neg(self) -> Self::Output {
        out!(vref_neg(@self))
    }
}

impl Not for VarRef {
    type Output = Result<VarRef>;

    fn not(self) -> Self::Output {
        out!(vref_not(@self))
    }
}

impl Not for &VarRef {
    type Output = Result<VarRef>;

    fn not(self) -> Self::Output {
        out!(vref_not(@self))
    }
}

// FAILABLE

// ADD

impl Add<VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn add(self, rhs: VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_add_vref(@slf, @rhs))
    }
}

impl Add<Result<VarRef>> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_add_vref(@self, @rhs))
    }
}

impl Add<Result<VarRef>> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_add_vref(@self, @rhs))
    }
}

impl Add<&VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_add_vref(@slf, @rhs))
    }
}

// SUB

impl Sub<VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_sub_vref(@slf, @rhs))
    }
}

impl Sub<Result<VarRef>> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_sub_vref(@self, @rhs))
    }
}

impl Sub<Result<VarRef>> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_sub_vref(@self, @rhs))
    }
}

impl Sub<&VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_sub_vref(@slf, @rhs))
    }
}

// MUL

impl Mul<VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn mul(self, rhs: VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_mul_vref(@slf, @rhs))
    }
}

impl Mul<Result<VarRef>> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_mul_vref(@self, @rhs))
    }
}

impl Mul<Result<VarRef>> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_mul_vref(@self, @rhs))
    }
}

impl Mul<&VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_mul_vref(@slf, @rhs))
    }
}


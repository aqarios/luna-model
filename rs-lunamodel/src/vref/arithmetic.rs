use std::ops::{Add, Mul, Sub};
use std::ops::{Neg, Not};

use super::VarRef;
use crate::Expression;

use crate::Result;

impl VarRef {
    /// Raises the variable to the `n`-th power.
    pub fn pow(&self, n: usize) -> Result<Expression> {
        out!(vref_pow(@self, n), |o| Ok(Expression::from_raw(o)))
    }

    /// Evaluates whether `value` satisfies the variable's bounds, within an
    /// optional tolerance. This is a metadata check against the variable's bound
    /// interval and does not inspect any expression context.
    pub fn evaluate(&self, value: f64, tol: Option<f64>) -> Result<bool> {
        out!(vref_eval(@self, value, ?tol))
    }
}

// ADD

impl Add<VarRef> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: VarRef) -> Self::Output {
        out!(vref_add_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Add<&VarRef> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        out!(vref_add_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Add<VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: VarRef) -> Self::Output {
        out!(vref_add_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Add<&VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        out!(vref_add_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Add<f64> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: f64) -> Self::Output {
        out!(vref_add_bias(@self, rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Add<f64> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: f64) -> Self::Output {
        out!(vref_add_bias(@self, rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Add<usize> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: usize) -> Self::Output {
        out!(vref_add_bias(@self, rhs as f64), |o| Ok(Expression::from_raw(o)))
    }
}

impl Add<usize> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: usize) -> Self::Output {
        out!(vref_add_bias(@self, rhs as f64), |o| Ok(Expression::from_raw(o)))
    }
}

// SUB

impl Sub<VarRef> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        out!(vref_sub_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<&VarRef> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        out!(vref_sub_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        out!(vref_sub_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<&VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        out!(vref_sub_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<f64> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: f64) -> Self::Output {
        out!(vref_sub_bias(@self, rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<f64> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: f64) -> Self::Output {
        out!(vref_sub_bias(@self, rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<usize> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: usize) -> Self::Output {
        out!(vref_sub_bias(@self, rhs as f64), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<usize> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: usize) -> Self::Output {
        out!(vref_sub_bias(@self, rhs as f64), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<VarRef> for f64 {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        out!(vref_rsub_bias(@rhs, self), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<&VarRef> for f64 {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        out!(vref_rsub_bias(@rhs, self), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<VarRef> for usize {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        out!(vref_rsub_bias(@rhs, self as f64), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<&VarRef> for usize {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        out!(vref_rsub_bias(@rhs, self as f64), |o| Ok(Expression::from_raw(o)))
    }
}

// MUL

impl Mul<VarRef> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: VarRef) -> Self::Output {
        out!(vref_mul_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Mul<&VarRef> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        out!(vref_mul_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Mul<VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: VarRef) -> Self::Output {
        out!(vref_mul_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Mul<&VarRef> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        out!(vref_mul_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Mul<f64> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: f64) -> Self::Output {
        out!(vref_mul_bias(@self, rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Mul<f64> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: f64) -> Self::Output {
        out!(vref_mul_bias(@self, rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Mul<usize> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: usize) -> Self::Output {
        out!(vref_mul_bias(@self, rhs as f64), |o| Ok(Expression::from_raw(o)))
    }
}

impl Mul<usize> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: usize) -> Self::Output {
        out!(vref_mul_bias(@self, rhs as f64), |o| Ok(Expression::from_raw(o)))
    }
}

impl Neg for VarRef {
    type Output = Result<Expression>;

    fn neg(self) -> Self::Output {
        out!(vref_neg(@self), |o| Ok(Expression::from_raw(o)))
    }
}

impl Not for VarRef {
    type Output = Result<VarRef>;

    fn not(self) -> Self::Output {
        out!(vref_not(@self), |o| Ok(VarRef::from_raw(o)))
    }
}

// FAILABLE

// ADD

impl Add<VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn add(self, rhs: VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_add_vref(@slf, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

// impl Add<Result<VarRef>> for Result<VarRef> {
//     type Output = Result<Expression>;
//
//     fn add(self, rhs: Result<VarRef>) -> Self::Output {
//         let slf = self?;
//         let rhs = rhs?;
//         out!(vref_add_vref(@slf, @rhs), |o| Ok(Expression::from_raw(o)))
//     }
// }

impl Add<Result<VarRef>> for VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_add_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Add<Result<VarRef>> for &VarRef {
    type Output = Result<Expression>;

    fn add(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_add_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Add<&VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_add_vref(@slf, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

// impl Add<f64> for Result<VarRef> {
//     type Output = Result<Expression>;
//
//     fn add(self, rhs: f64) -> Self::Output {
//         let slf = self?;
//         out!(vref_add_bias(@slf, rhs), |o| Ok(Expression::from_raw(o)))
//     }
// }
//
// impl Add<usize> for Result<VarRef> {
//     type Output = Result<Expression>;
//
//     fn add(self, rhs: usize) -> Self::Output {
//         let slf = self?;
//         out!(vref_add_bias(@slf, rhs as f64), |o| Ok(Expression::from_raw(o)))
//     }
// }

// SUB

impl Sub<VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn sub(self, rhs: VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_sub_vref(@slf, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

// impl Sub<Result<VarRef>> for Result<VarRef> {
//     type Output = Result<Expression>;
//
//     fn sub(self, rhs: Result<VarRef>) -> Self::Output {
//         let slf = self?;
//         let rhs = rhs?;
//         out!(vref_sub_vref(@slf, @rhs), |o| Ok(Expression::from_raw(o)))
//     }
// }

impl Sub<Result<VarRef>> for VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_sub_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<Result<VarRef>> for &VarRef {
    type Output = Result<Expression>;

    fn sub(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_sub_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Sub<&VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn sub(self, rhs: &VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_sub_vref(@slf, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

// impl Sub<f64> for Result<VarRef> {
//     type Output = Result<Expression>;
//
//     fn sub(self, rhs: f64) -> Self::Output {
//         let slf = self?;
//         out!(vref_sub_bias(@slf, rhs), |o| Ok(Expression::from_raw(o)))
//     }
// }
//
// impl Sub<usize> for Result<VarRef> {
//     type Output = Result<Expression>;
//
//     fn sub(self, rhs: usize) -> Self::Output {
//         let slf = self?;
//         out!(vref_sub_bias(@slf, rhs as f64), |o| Ok(Expression::from_raw(o)))
//     }
// }
//
// impl Sub<Result<VarRef>> for f64 {
//     type Output = Result<Expression>;
//
//     fn sub(self, rhs: Result<VarRef>) -> Self::Output {
//         let rhs = rhs?;
//         out!(vref_rsub_bias(@rhs, self), |o| Ok(Expression::from_raw(o)))
//     }
// }
//
// impl Sub<Result<VarRef>> for usize {
//     type Output = Result<Expression>;
//
//     fn sub(self, rhs: Result<VarRef>) -> Self::Output {
//         let rhs = rhs?;
//         out!(vref_rsub_bias(@rhs, self as f64), |o| Ok(Expression::from_raw(o)))
//     }
// }

// MUL

impl Mul<VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn mul(self, rhs: VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_mul_vref(@slf, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

// impl Mul<Result<VarRef>> for Result<VarRef> {
//     type Output = Result<Expression>;
//
//     fn mul(self, rhs: Result<VarRef>) -> Self::Output {
//         let slf = self?;
//         let rhs = rhs?;
//         out!(vref_mul_vref(@slf, @rhs), |o| Ok(Expression::from_raw(o)))
//     }
// }

impl Mul<Result<VarRef>> for VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_mul_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Mul<Result<VarRef>> for &VarRef {
    type Output = Result<Expression>;

    fn mul(self, rhs: Result<VarRef>) -> Self::Output {
        let rhs = rhs?;
        out!(vref_mul_vref(@self, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

impl Mul<&VarRef> for Result<VarRef> {
    type Output = Result<Expression>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        let slf = self?;
        out!(vref_mul_vref(@slf, @rhs), |o| Ok(Expression::from_raw(o)))
    }
}

// impl Mul<f64> for Result<VarRef> {
//     type Output = Result<Expression>;
//
//     fn mul(self, rhs: f64) -> Self::Output {
//         let slf = self?;
//         out!(vref_mul_bias(@slf, rhs), |o| Ok(Expression::from_raw(o)))
//     }
// }
//
// impl Mul<usize> for Result<VarRef> {
//     type Output = Result<Expression>;
//
//     fn mul(self, rhs: usize) -> Self::Output {
//         let slf = self?;
//         out!(vref_mul_bias(@slf, rhs as f64), |o| Ok(Expression::from_raw(o)))
//     }
// }
//
// impl Neg for Result<VarRef> {
//     type Output = Result<Expression>;
//
//     fn neg(self) -> Self::Output {
//         let slf = self?;
//         out!(vref_neg(@slf), |o| Ok(Expression::from_raw(o)))
//     }
// }
//
// impl Not for Result<VarRef> {
//     type Output = Result<VarRef>;
//
//     fn not(self) -> Self::Output {
//         let slf = self?;
//         out!(vref_not(@slf), |o| Ok(VarRef::from_raw(o)))
//     }
// }

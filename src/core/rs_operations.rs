use core::fmt;
use std::{
    error::Error,
    fmt::Display,
    ops::{Add, BitXor, Mul},
};

use num::pow::Pow;

use crate::{
    core::{operations::AddToExpression, Expression},
    errors::{UnsupportedOperationErr, VariablesFromDifferentEnvsErr},
};

use super::{
    expression::ExpressionBaseCreation,
    operations::{MulAssignToExpression, MulToExpression},
    VarRef,
};

fn unwrap_failed(msg: &str, error: &dyn fmt::Debug) -> ! {
    panic!("{msg}: {error:?}")
}

pub enum AqResult<T, E> {
    Err(E),
    Ok(T),
}

impl<T, E> AqResult<T, E> {
    pub fn into_result(self) -> Result<T, E> {
        match self {
            Self::Ok(elem) => Ok(elem),
            Self::Err(err) => Err(err),
        }
    }

    pub fn expect(self, msg: &str) -> T
    where
        E: fmt::Debug,
    {
        match self {
            AqResult::Ok(t) => t,
            AqResult::Err(e) => unwrap_failed(msg, &e),
        }
    }
}

impl<T, E, O> From<Result<T, E>> for AqResult<T, O>
where
    O: From<E>,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Err(err) => Self::Err(err.into()),
            Ok(val) => Self::Ok(val),
        }
    }
}

#[derive(Debug)]
pub enum OperationErr {
    VariablesFromDifferentEnvs(VariablesFromDifferentEnvsErr),
    UnsupportedOperation(UnsupportedOperationErr),
}
impl Error for OperationErr {}
impl Display for OperationErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            OperationErr::VariablesFromDifferentEnvs(err) => err.to_string(),
            OperationErr::UnsupportedOperation(err) => err.to_string(),
        };
        write!(f, "{}", msg)
    }
}

impl From<VariablesFromDifferentEnvsErr> for OperationErr {
    fn from(value: VariablesFromDifferentEnvsErr) -> Self {
        Self::VariablesFromDifferentEnvs(value)
    }
}

impl From<UnsupportedOperationErr> for OperationErr {
    fn from(value: UnsupportedOperationErr) -> Self {
        Self::UnsupportedOperation(value)
    }
}

// pub type OperationResult<T> = AqResult<T, OperationErr>;
// note: This is not ideal.. I don't really want to force return an expression here.
// But for this the error forwarding operations implemented in this file need to be
// rewritten to support some generic type supporting the operation on that generic type
// used internally. Once that is done, we can return the inv directly instead of
// creating a new linear expression.
//
// Needed since the [Not] operation on a [VarRef] returns another [VarRef] not an Expression.
// The [Not] operation must remain as is on the &VarRef (or VarRef (to be added)) and the other
// operations should be capable of handling such a specific error type... Maybe we even need to
// remove the speical AqResult and replace it with the actual Result type...
pub type OperationResult = AqResult<Expression, OperationErr>;

impl Add<f64> for &VarRef {
    type Output = Expression;

    fn add(self, rhs: f64) -> Self::Output {
        <Self as AddToExpression<f64>>::add(self, rhs)
    }
}

impl Add<VarRef> for f64 {
    type Output = Expression;

    fn add(self, rhs: VarRef) -> Self::Output {
        <&VarRef as AddToExpression<f64>>::add(&rhs, self)
    }
}

impl Add<&VarRef> for f64 {
    type Output = Expression;

    fn add(self, rhs: &VarRef) -> Self::Output {
        <&VarRef as AddToExpression<f64>>::add(rhs, self)
    }
}

impl Add<f64> for VarRef {
    type Output = Expression;

    fn add(self, rhs: f64) -> Self::Output {
        <&Self as AddToExpression<f64>>::add(&self, rhs)
    }
}

impl Add for &VarRef {
    type Output = OperationResult;

    fn add(self, rhs: Self) -> Self::Output {
        <Self as AddToExpression<Self>>::add(self, rhs).into()
    }
}

impl Add for VarRef {
    type Output = OperationResult;

    fn add(self, rhs: Self) -> Self::Output {
        <&Self as AddToExpression<&Self>>::add(&self, &rhs).into()
    }
}

impl Add<OperationResult> for &VarRef {
    type Output = OperationResult;

    fn add(self, rhs: OperationResult) -> Self::Output {
        match rhs {
            AqResult::Err(e) => AqResult::Err(e),
            AqResult::Ok(expr) => <&Expression as AddToExpression<Self>>::add(&expr, self).into(),
        }
    }
}

impl Add<Expression> for &VarRef {
    type Output = OperationResult;

    fn add(self, rhs: Expression) -> Self::Output {
        <&Expression as AddToExpression<&VarRef>>::add(&rhs, self).into()
    }
}

impl Add<VarRef> for OperationResult {
    type Output = OperationResult;

    fn add(self, rhs: VarRef) -> Self::Output {
        match self {
            AqResult::Err(e) => AqResult::Err(e),
            AqResult::Ok(expr) => {
                <&Expression as AddToExpression<&VarRef>>::add(&expr, &rhs).into()
            }
        }
    }
}

impl Add<&VarRef> for OperationResult {
    type Output = OperationResult;

    fn add(self, rhs: &VarRef) -> Self::Output {
        match self {
            AqResult::Err(e) => AqResult::Err(e),
            AqResult::Ok(expr) => <&Expression as AddToExpression<&VarRef>>::add(&expr, rhs).into(),
        }
    }
}

impl Add<Expression> for Expression {
    type Output = OperationResult;

    fn add(self, rhs: Expression) -> Self::Output {
        <&Expression as AddToExpression<&Expression>>::add(&self, &rhs).into()
    }
}

impl Add<&Expression> for &Expression {
    type Output = OperationResult;

    fn add(self, rhs: &Expression) -> Self::Output {
        <&Expression as AddToExpression<&Expression>>::add(self, rhs).into()
    }
}

impl Add<&VarRef> for Expression {
    type Output = OperationResult;

    fn add(self, rhs: &VarRef) -> Self::Output {
        <&Expression as AddToExpression<&VarRef>>::add(&self, rhs).into()
    }
}

impl Add<OperationResult> for OperationResult {
    type Output = OperationResult;

    fn add(self, rhs: OperationResult) -> Self::Output {
        match self {
            AqResult::Err(err) => AqResult::Err(err),
            AqResult::Ok(expr) => match rhs {
                AqResult::Err(err) => AqResult::Err(err),
                AqResult::Ok(rhsexpr) => {
                    <&Expression as AddToExpression<&Expression>>::add(&expr, &rhsexpr).into()
                }
            },
        }
    }
}

impl Add<OperationResult> for Expression {
    type Output = OperationResult;

    fn add(self, rhs: OperationResult) -> Self::Output {
        match rhs {
            AqResult::Err(err) => AqResult::Err(err),
            AqResult::Ok(rhsexpr) => {
                <&Expression as AddToExpression<&Expression>>::add(&self, &rhsexpr).into()
            }
        }
    }
}

impl Add<Expression> for OperationResult {
    type Output = OperationResult;

    fn add(self, rhs: Expression) -> Self::Output {
        match self {
            AqResult::Err(err) => AqResult::Err(err),
            AqResult::Ok(expr) => {
                <&Expression as AddToExpression<&Expression>>::add(&expr, &rhs).into()
            }
        }
    }
}

impl Mul<f64> for &VarRef {
    type Output = Expression;

    fn mul(self, rhs: f64) -> Self::Output {
        <Self as MulToExpression<f64>>::mul(self, rhs)
    }
}

impl Mul<f64> for VarRef {
    type Output = Expression;

    fn mul(self, rhs: f64) -> Self::Output {
        <&Self as MulToExpression<f64>>::mul(&self, rhs)
    }
}

impl Mul<usize> for VarRef {
    type Output = Expression;

    fn mul(self, rhs: usize) -> Self::Output {
        <&Self as MulToExpression<f64>>::mul(&self, rhs as f64)
    }
}

impl Mul<&VarRef> for usize {
    type Output = Expression;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        <&VarRef as MulToExpression<f64>>::mul(&rhs, self as f64)
    }
}

impl Mul<VarRef> for f64 {
    type Output = Expression;

    fn mul(self, rhs: VarRef) -> Self::Output {
        <&VarRef as MulToExpression<f64>>::mul(&rhs, self)
    }
}

impl Mul<&VarRef> for f64 {
    type Output = Expression;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        <&VarRef as MulToExpression<f64>>::mul(rhs, self)
    }
}

impl Mul for &VarRef {
    type Output = OperationResult;

    fn mul(self, rhs: Self) -> Self::Output {
        <Self as MulToExpression<Self>>::mul(self, rhs).into()
    }
}

impl Mul for VarRef {
    type Output = OperationResult;

    fn mul(self, rhs: Self) -> Self::Output {
        <&Self as MulToExpression<&Self>>::mul(&self, &rhs).into()
    }
}

impl Mul<Expression> for VarRef {
    type Output = OperationResult;

    fn mul(self, rhs: Expression) -> Self::Output {
        <&Expression as MulToExpression<&Self>>::mul(&rhs, &self).into()
    }
}

impl Mul<Expression> for &VarRef {
    type Output = OperationResult;

    fn mul(self, rhs: Expression) -> Self::Output {
        <&Expression as MulToExpression<Self>>::mul(&rhs, self).into()
    }
}

impl Mul<&VarRef> for Expression {
    type Output = OperationResult;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        <&Self as MulToExpression<&VarRef>>::mul(&self, rhs).into()
    }
}

impl Mul<OperationResult> for &VarRef {
    type Output = OperationResult;

    fn mul(self, rhs: OperationResult) -> Self::Output {
        match rhs {
            AqResult::Err(e) => AqResult::Err(e),
            AqResult::Ok(expr) => <&Expression as MulToExpression<Self>>::mul(&expr, self).into(),
        }
    }
}

impl Mul<&VarRef> for OperationResult {
    type Output = OperationResult;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        match self {
            AqResult::Err(e) => AqResult::Err(e),
            AqResult::Ok(expr) => <&Expression as MulToExpression<&VarRef>>::mul(&expr, rhs).into(),
        }
    }
}

impl Mul<VarRef> for OperationResult {
    type Output = OperationResult;

    fn mul(self, rhs: VarRef) -> Self::Output {
        match self {
            AqResult::Err(e) => AqResult::Err(e),
            AqResult::Ok(expr) => {
                <&Expression as MulToExpression<&VarRef>>::mul(&expr, &rhs).into()
            }
        }
    }
}

impl Mul<OperationResult> for f64 {
    type Output = OperationResult;

    fn mul(self, rhs: OperationResult) -> Self::Output {
        match rhs {
            AqResult::Err(e) => AqResult::Err(e),
            AqResult::Ok(expr) => {
                AqResult::Ok(<&Expression as MulToExpression<f64>>::mul(&expr, self))
            }
        }
    }
}

impl Mul<OperationResult> for usize {
    type Output = OperationResult;

    fn mul(self, rhs: OperationResult) -> Self::Output {
        match rhs {
            AqResult::Err(e) => AqResult::Err(e),
            AqResult::Ok(expr) => AqResult::Ok(<&Expression as MulToExpression<f64>>::mul(
                &expr,
                self as f64,
            )),
        }
    }
}

// impl Not for VarRef {
//     type Output = OperationResult;
//
//     fn not(self) -> Self::Output {
//         match (&self).not() {
//             // note: This is not ideal.. I don't really want to return an expression here. But for
//             // this the other error forwarding operations implemented in this file need to be
//             // rewritten to support some generic type supporting the operation on that generic type
//             // used internally. Once that is done, we can return the inv directly instead of
//             // creating a new linear expression.
//             Ok(inv) => AqResult::Ok(Expression::new_linear_single(
//                 self.env.clone(),
//                 inv.id,
//                 Bias::one(),
//             )),
//             Err(e) => AqResult::Err(e.into()),
//         }
//     }
// }

//impl Mul<OperationResult> for f64 {
//    type Output = OperationResult;
//
//    fn mul(self, rhs: OperationResult) -> Self::Output {
//        match rhs {
//            Err(e) => Err(e),
//            Ok(expr) => Ok(<&Expression as MulToExpression<f64>>::mul(&expr, self)),
//        }
//    }
//}

//impl Mul<OperationResult> for f64 {
//    type Output = OperationResult;
//
//    fn mul(self, rhs: OperationResult) -> Self::Output {
//        match rhs {
//            Err(e) => Err(e),
//            Ok(expr) => Ok(<&Expression as MulToExpression<f64>>::mul(&expr, self)),
//        }
//    }
//}
//
//
//
//
impl Pow<usize> for &VarRef {
    type Output = OperationResult;

    fn pow(self, rhs: usize) -> Self::Output {
        match rhs {
            0 => AqResult::Ok(Expression::simple(self.env.clone(), 1.0)),
            1 => AqResult::Ok(Expression::new_linear_single(
                self.env.clone(),
                self.id,
                1.0,
            )),
            2 => AqResult::Ok(Expression::new_quadratic(
                self.env.clone(),
                self.id,
                self.id,
                1.0,
            )),
            _ => {
                let mut out = Expression::simple(self.env.clone(), 1.0);
                for _ in 0..rhs {
                    match out.mul_assign(self) {
                        Ok(_) => (),
                        Err(err) => return AqResult::Err(err.into()),
                    }
                }
                AqResult::Ok(out)
            }
        }
    }
}

impl BitXor<usize> for &VarRef {
    type Output = OperationResult;

    fn bitxor(self, rhs: usize) -> Self::Output {
        self.pow(rhs)
    }
}

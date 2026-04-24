//! Shared storage for Python expressions.
//!
//! `PyExpression` sometimes owns a standalone [`Expression`] and sometimes
//! provides a mutable view onto a model objective. `PyExprContent` abstracts
//! over those two cases so the Python API can expose one expression wrapper
//! without forcing model objectives to be eagerly copied out of the model.

use lunamodel_core::{
    ops::{LmAddAssign, LmMulAssign, LmPow, LmPowAssign, LmSubAssign},
    prelude::{ContentEquality, Expression, Model, VarRef},
};
use lunamodel_error::LunaModelResult;
use lunamodel_io::{CustomFormat, FormatOpt};
use parking_lot::RwLock;
use std::{
    ops::{Add, Mul, Neg, Sub},
    sync::Arc,
};

#[derive(Debug)]
pub enum PyExprContent {
    /// A standalone expression wrapper backed by its own lock.
    Expr(Arc<RwLock<Expression>>),
    /// A borrowed view onto a model whose objective acts as the expression.
    Model(Arc<RwLock<Model>>),
}

impl Clone for PyExprContent {
    fn clone(&self) -> Self {
        match self {
            Self::Expr(e) => Self::Expr(e.clone()),
            Self::Model(e) => Self::Model(e.clone()),
        }
    }
}

impl From<PyExprContent> for Expression {
    /// Detach the Python content from its wrapper and clone out an owned
    /// expression.
    ///
    /// Model-backed content clones the current objective, which intentionally
    /// breaks the live link to the source model.
    fn from(val: PyExprContent) -> Self {
        match val {
            PyExprContent::Expr(e) => e.read_arc().clone(),
            PyExprContent::Model(m) => m.read_arc().objective.clone(),
        }
    }
}

impl PyExprContent {
    /// Evaluate addition against the wrapped expression without exposing lock
    /// guards to the caller.
    pub fn add<T>(&self, other: T) -> LunaModelResult<Expression>
    where
        for<'e> &'e Expression: Add<T, Output = LunaModelResult<Expression>>,
    {
        self.read_with(|e| e.add(other))
    }

    /// Evaluate subtraction against the wrapped expression.
    pub fn sub<T>(&self, other: T) -> LunaModelResult<Expression>
    where
        for<'e> &'e Expression: Sub<T, Output = LunaModelResult<Expression>>,
    {
        self.read_with(|e| e.sub(other))
    }

    /// Evaluate multiplication against the wrapped expression.
    pub fn mul<T>(&self, other: T) -> LunaModelResult<Expression>
    where
        for<'e> &'e Expression: Mul<T, Output = LunaModelResult<Expression>>,
    {
        self.read_with(|e| e.mul(other))
    }

    /// Raise the wrapped expression to an integer power.
    pub fn pow(&self, v: usize) -> LunaModelResult<Expression> {
        self.read_with(|e| e.pow(v))
    }

    /// Mutate the wrapped expression in place by adding `other`.
    pub fn add_assign<T>(&mut self, other: T) -> LunaModelResult<()>
    where
        Expression: LmAddAssign<T>,
    {
        self.write_with(|e| e.add_assign(other))
    }

    /// Mutate the wrapped expression in place by subtracting `other`.
    pub fn sub_assign<T>(&mut self, other: T) -> LunaModelResult<()>
    where
        Expression: LmSubAssign<T>,
    {
        self.write_with(|e| e.sub_assign(other))
    }

    /// Mutate the wrapped expression in place by multiplying by `other`.
    pub fn mul_assign<T>(&mut self, other: T) -> LunaModelResult<()>
    where
        Expression: LmMulAssign<T>,
    {
        self.write_with(|e| e.mul_assign(other))
    }

    /// Mutate the wrapped expression in place by repeated multiplication.
    pub fn pow_assign(&mut self, v: usize) -> LunaModelResult<()> {
        self.write_with(|e| e.pow_assign(v))
    }

    /// Substitute one variable reference with another expression.
    ///
    /// For model-backed content this rewrites the live objective in place when
    /// used through the assignment-oriented APIs.
    pub fn substitute(
        &self,
        target: &VarRef,
        replacement: &Expression,
    ) -> LunaModelResult<Expression> {
        self.read_with(|e| e.substitute(target, replacement))
    }
}

impl Add<&PyExprContent> for &Expression {
    type Output = LunaModelResult<Expression>;
    fn add(self, rhs: &PyExprContent) -> Self::Output {
        rhs.read_with(|rhs| self.add(rhs))
    }
}

impl Mul<&PyExprContent> for &Expression {
    type Output = LunaModelResult<Expression>;
    fn mul(self, rhs: &PyExprContent) -> Self::Output {
        rhs.read_with(|rhs| self.mul(rhs))
    }
}

impl Sub<&PyExprContent> for &Expression {
    type Output = LunaModelResult<Expression>;
    fn sub(self, rhs: &PyExprContent) -> Self::Output {
        rhs.read_with(|rhs| self.sub(rhs))
    }
}
impl Sub<&PyExprContent> for Expression {
    type Output = LunaModelResult<Expression>;
    fn sub(self, rhs: &PyExprContent) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl Neg for &PyExprContent {
    type Output = Expression;
    fn neg(self) -> Self::Output {
        self.read_with(|slf| slf.neg())
    }
}

impl LmAddAssign<&PyExprContent> for Expression {
    fn add_assign(&mut self, rhs: &PyExprContent) -> LunaModelResult<()> {
        rhs.read_with(|rhs| self.add_assign(rhs))
    }
}

impl LmSubAssign<&PyExprContent> for Expression {
    fn sub_assign(&mut self, rhs: &PyExprContent) -> LunaModelResult<()> {
        rhs.read_with(|rhs| self.sub_assign(rhs))
    }
}

impl LmMulAssign<&PyExprContent> for Expression {
    fn mul_assign(&mut self, rhs: &PyExprContent) -> LunaModelResult<()> {
        rhs.read_with(|rhs| self.mul_assign(rhs))
    }
}

impl PartialEq for PyExprContent {
    fn eq(&self, other: &Self) -> bool {
        let slf: &Expression = &self.read();
        let otr: &Expression = &other.read();
        slf.eq(otr)
    }
}

impl Sub<&PyExprContent> for &VarRef {
    type Output = LunaModelResult<Expression>;

    fn sub(self, rhs: &PyExprContent) -> Self::Output {
        rhs.read_with(|e| self.sub(e))
    }
}

impl ContentEquality for PyExprContent {
    fn equal_contents(&self, other: &Self) -> bool {
        let slf: &Expression = &self.read();
        let otr: &Expression = &other.read();
        slf.equal_contents(otr)
    }
}

impl CustomFormat<FormatOpt> for PyExprContent {
    /// Delegate formatting to the current expression view.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        self.read_with(|e| e.fmt(f, format_type))
    }

    /// Delegate debug formatting to the current expression view.
    fn dbg(&self, f: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        self.read_with(|e| e.dbg(f, format_type))
    }
}

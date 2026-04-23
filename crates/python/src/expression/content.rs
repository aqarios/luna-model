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
    Expr(Arc<RwLock<Expression>>),
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

impl Into<Expression> for PyExprContent {
    fn into(self) -> Expression {
        match self {
            Self::Expr(e) => e.read_arc().clone(),
            Self::Model(m) => m.read_arc().objective.clone(),
        }
    }
}

impl PyExprContent {
    pub fn add<T>(&self, other: T) -> LunaModelResult<Expression>
    where
        for<'e> &'e Expression: Add<T, Output = LunaModelResult<Expression>>,
    {
        self.read_with(|e| e.add(other))
    }

    pub fn sub<T>(&self, other: T) -> LunaModelResult<Expression>
    where
        for<'e> &'e Expression: Sub<T, Output = LunaModelResult<Expression>>,
    {
        self.read_with(|e| e.sub(other))
    }

    pub fn mul<T>(&self, other: T) -> LunaModelResult<Expression>
    where
        for<'e> &'e Expression: Mul<T, Output = LunaModelResult<Expression>>,
    {
        self.read_with(|e| e.mul(other))
    }

    pub fn pow(&self, v: usize) -> LunaModelResult<Expression> {
        self.read_with(|e| e.pow(v))
    }

    pub fn add_assign<T>(&mut self, other: T) -> LunaModelResult<()>
    where
        Expression: LmAddAssign<T>,
    {
        self.write_with(|e| e.add_assign(other))
    }

    pub fn sub_assign<T>(&mut self, other: T) -> LunaModelResult<()>
    where
        Expression: LmSubAssign<T>,
    {
        self.write_with(|e| e.sub_assign(other))
    }

    pub fn mul_assign<T>(&mut self, other: T) -> LunaModelResult<()>
    where
        Expression: LmMulAssign<T>,
    {
        self.write_with(|e| e.mul_assign(other))
    }

    pub fn pow_assign(&mut self, v: usize) -> LunaModelResult<()> {
        self.write_with(|e| e.pow_assign(v))
    }

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        self.read_with(|e| e.fmt(f, format_type))
    }

    fn dbg(&self, f: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        self.read_with(|e| e.dbg(f, format_type))
    }
}

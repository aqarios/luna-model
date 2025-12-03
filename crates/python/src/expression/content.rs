use lunamodel_core::{
    ops::{LmAddAssign, LmMulAssign, LmPowAssign, LmSubAssign, LmPow},
    prelude::{Expression, Model},
};
use lunamodel_error::LunaModelResult;
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

impl PyExprContent {
    pub fn add<T>(&self, other: T) -> LunaModelResult<Expression>
    where
        for<'e> &'e Expression: Add<T, Output = LunaModelResult<Expression>>,
    {
        match self {
            PyExprContent::Expr(expr) => {
                let slf: &Expression = &expr.read_arc();
                slf.add(other)
            }
            PyExprContent::Model(model) => {
                let slf: &Expression = &model.read_arc().objective;
                slf.add(other)
            }
        }
    }

    pub fn sub<T>(&self, other: T) -> LunaModelResult<Expression>
    where
        for<'e> &'e Expression: Sub<T, Output = LunaModelResult<Expression>>,
    {
        match self {
            PyExprContent::Expr(expr) => {
                let slf: &Expression = &expr.read_arc();
                slf.sub(other)
            }
            PyExprContent::Model(model) => {
                let slf: &Expression = &model.read_arc().objective;
                slf.sub(other)
            }
        }
    }

    pub fn mul<T>(&self, other: T) -> LunaModelResult<Expression>
    where
        for<'e> &'e Expression: Mul<T, Output = LunaModelResult<Expression>>,
    {
        match self {
            PyExprContent::Expr(expr) => {
                let slf: &Expression = &expr.read_arc();
                slf.mul(other)
            }
            PyExprContent::Model(model) => {
                let slf: &Expression = &model.read_arc().objective;
                slf.mul(other)
            }
        }
    }

    pub fn pow(&self, v: usize) -> LunaModelResult<Expression> {
        match self {
            PyExprContent::Expr(expr) => {
                let slf: &Expression = &expr.read_arc();
                slf.pow(v)
            }
            PyExprContent::Model(model) => {
                let slf: &Expression = &model.read_arc().objective;
                slf.pow(v)
            }
        }
    }

    pub fn add_assign<T>(&self, other: T) -> LunaModelResult<()>
    where
        Expression: LmAddAssign<T>,
    {
        match self {
            PyExprContent::Expr(expr) => {
                let slf: &mut Expression = &mut expr.write_arc();
                slf.add_assign(other)
            }
            PyExprContent::Model(model) => {
                let slf: &mut Expression = &mut model.write_arc().objective;
                slf.add_assign(other)
            }
        }
    }

    pub fn sub_assign<T>(&self, other: T) -> LunaModelResult<()>
    where
        Expression: LmSubAssign<T>,
    {
        match self {
            PyExprContent::Expr(expr) => {
                let slf: &mut Expression = &mut expr.write_arc();
                slf.sub_assign(other)
            }
            PyExprContent::Model(model) => {
                let slf: &mut Expression = &mut model.write_arc().objective;
                slf.sub_assign(other)
            }
        }
    }

    pub fn mul_assign<T>(&self, other: T) -> LunaModelResult<()>
    where
        Expression: LmMulAssign<T>,
    {
        match self {
            PyExprContent::Expr(expr) => {
                let slf: &mut Expression = &mut expr.write_arc();
                slf.mul_assign(other)
            }
            PyExprContent::Model(model) => {
                let slf: &mut Expression = &mut model.write_arc().objective;
                slf.mul_assign(other)
            }
        }
    }

    pub fn pow_assign(&self, v: usize) -> LunaModelResult<()> {
        match self {
            PyExprContent::Expr(expr) => {
                let slf: &mut Expression = &mut expr.write_arc();
                slf.pow_assign(v)
            }
            PyExprContent::Model(model) => {
                let slf: &mut Expression = &mut model.write_arc().objective;
                slf.pow_assign(v)
            }
        }
    }
}

impl Add<&PyExprContent> for &Expression {
    type Output = LunaModelResult<Expression>;
    fn add(self, rhs: &PyExprContent) -> Self::Output {
        match rhs {
            PyExprContent::Expr(r) => {
                let rs: &Expression = &r.read_arc();
                self.add(rs)
            }
            PyExprContent::Model(r) => {
                let rs: &Expression = &r.read_arc().objective;
                self.add(rs)
            }
        }
    }
}

impl Mul<&PyExprContent> for &Expression {
    type Output = LunaModelResult<Expression>;
    fn mul(self, rhs: &PyExprContent) -> Self::Output {
        match rhs {
            PyExprContent::Expr(r) => {
                let rs: &Expression = &r.read_arc();
                self.mul(rs)
            }
            PyExprContent::Model(r) => {
                let rs: &Expression = &r.read_arc().objective;
                self.mul(rs)
            }
        }
    }
}

impl Sub<&PyExprContent> for &Expression {
    type Output = LunaModelResult<Expression>;
    fn sub(self, rhs: &PyExprContent) -> Self::Output {
        match rhs {
            PyExprContent::Expr(r) => {
                let rs: &Expression = &r.read_arc();
                self.sub(rs)
            }
            PyExprContent::Model(r) => {
                let rs: &Expression = &r.read_arc().objective;
                self.sub(rs)
            }
        }
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
        match self {
            PyExprContent::Expr(r) => {
                let rs: &Expression = &r.read_arc();
                rs.neg()
            }
            PyExprContent::Model(r) => {
                let rs: &Expression = &r.read_arc().objective;
                rs.neg()
            }
        }
    }
}

impl LmAddAssign<&PyExprContent> for Expression {
    fn add_assign(&mut self, rhs: &PyExprContent) -> LunaModelResult<()> {
        match rhs {
            PyExprContent::Expr(r) => {
                let rs: &Expression = &r.read_arc();
                self.add_assign(rs)
            }
            PyExprContent::Model(r) => {
                let rs: &Expression = &r.read_arc().objective;
                self.add_assign(rs)
            }
        }
    }
}

impl LmSubAssign<&PyExprContent> for Expression {
    fn sub_assign(&mut self, rhs: &PyExprContent) -> LunaModelResult<()> {
        match rhs {
            PyExprContent::Expr(r) => {
                let rs: &Expression = &r.read_arc();
                self.sub_assign(rs)
            }
            PyExprContent::Model(r) => {
                let rs: &Expression = &r.read_arc().objective;
                self.sub_assign(rs)
            }
        }
    }
}

impl LmMulAssign<&PyExprContent> for Expression {
    fn mul_assign(&mut self, rhs: &PyExprContent) -> LunaModelResult<()> {
        match rhs {
            PyExprContent::Expr(r) => {
                let rs: &Expression = &r.read_arc();
                self.mul_assign(rs)
            }
            PyExprContent::Model(r) => {
                let rs: &Expression = &r.read_arc().objective;
                self.mul_assign(rs)
            }
        }
    }
}

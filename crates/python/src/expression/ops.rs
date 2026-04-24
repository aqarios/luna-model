//! Python operator overloads for expressions.

use lunamodel_error::py::PyLunaModelError;
use lunamodel_unwind::*;
use pyo3::prelude::*;
use std::ops::Neg;

use crate::PyExpression;
use crate::utils::{OpsOther as OO, as_usize};

#[unwindable]
#[pymethods]
impl PyExpression {
    /// Implement `expr + rhs` for numbers, variables, and expressions.
    pub fn __add__(&self, rhs: OO) -> PyResult<Self> {
        let expr = match rhs {
            OO::Expr(expr) => self.expr.add(&expr.expr),
            OO::Var(var) => self.expr.add(&var.v),
            OO::Num(bias) => self.expr.add(bias),
        }?;
        Ok(Self::new(expr))
    }

    /// Implement `expr - rhs`.
    pub fn __sub__(&self, rhs: OO) -> PyResult<Self> {
        let expr = match rhs {
            OO::Expr(expr) => self.expr.sub(&expr.expr),
            OO::Var(var) => self.expr.sub(&var.v),
            OO::Num(bias) => self.expr.sub(bias),
        }?;
        Ok(Self::new(expr))
    }

    /// Implement `expr * rhs`.
    pub fn __mul__(&self, rhs: OO) -> PyResult<Self> {
        let expr = match rhs {
            OO::Expr(expr) => self.expr.mul(&expr.expr),
            OO::Var(var) => self.expr.mul(&var.v),
            OO::Num(bias) => self.expr.mul(bias),
        }?;
        Ok(Self::new(expr))
    }

    /// Implement right-hand addition by reusing normal addition semantics.
    pub fn __radd__(&self, lhs: OO) -> PyResult<Self> {
        self.__add__(lhs)
    }

    /// Implement right-hand subtraction as `lhs + (-self)`.
    pub fn __rsub__(&self, lhs: OO) -> PyResult<Self> {
        self.__neg__()?.__add__(lhs)
    }

    /// Implement right-hand multiplication by reusing normal multiplication semantics.
    pub fn __rmul__(&self, lhs: OO) -> PyResult<Self> {
        self.__mul__(lhs)
    }

    /// Implement in-place addition.
    pub fn __iadd__(&mut self, rhs: OO) -> PyResult<()> {
        match rhs {
            OO::Expr(expr) => self.expr.add_assign(&expr.expr),
            OO::Var(var) => self.expr.add_assign(&var.v),
            OO::Num(bias) => self.expr.add_assign(bias),
        }?;
        Ok(())
    }

    /// Implement in-place subtraction.
    pub fn __isub__(&mut self, rhs: OO) -> PyResult<()> {
        match rhs {
            OO::Expr(expr) => self.expr.sub_assign(&expr.expr),
            OO::Var(var) => self.expr.sub_assign(&var.v),
            OO::Num(bias) => self.expr.sub_assign(bias),
        }?;
        Ok(())
    }

    /// Implement in-place multiplication.
    pub fn __imul__(&mut self, rhs: OO) -> PyResult<()> {
        match rhs {
            OO::Expr(expr) => self.expr.mul_assign(&expr.expr),
            OO::Var(var) => self.expr.mul_assign(&var.v),
            OO::Num(bias) => self.expr.mul_assign(bias),
        }?;
        Ok(())
    }

    /// Implement exponentiation without Python's optional modulo operand.
    ///
    /// The binding accepts Python integers, converts them to `usize`, and then
    /// delegates to the Rust expression power implementation.
    pub fn __pow__(&mut self, other: isize, modulo: Option<isize>) -> PyResult<Self> {
        if modulo.is_some() {
            return Err(PyLunaModelError::new_err(
                "the 'modulo' parameter is not supported.",
            ));
        }
        Ok(Self::new(self.expr.pow(as_usize(other)?)?))
    }

    /// Implement in-place exponentiation without modulo support.
    pub fn __ipow__(&mut self, other: isize, modulo: Option<isize>) -> PyResult<()> {
        if modulo.is_some() {
            return Err(PyLunaModelError::new_err(
                "the 'modulo' parameter is not supported.",
            ));
        }
        Ok(self.expr.pow_assign(as_usize(other)?)?)
    }

    /// Return the negated expression.
    pub fn __neg__(&self) -> Self {
        Self::new(self.expr.neg())
    }
}

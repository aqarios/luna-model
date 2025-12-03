use lunamodel_error::py::PyLunaModelError;
use pyo3::prelude::*;
use std::ops::Neg;

use crate::PyExpression;
use crate::utils::OpsOther as OO;

#[pymethods]
impl PyExpression {
    pub fn __add__(&self, rhs: OO) -> PyResult<Self> {
        let expr = match rhs {
            OO::Expr(expr) => (&self.expr).add(&expr.expr),
            OO::Var(var) => (&self.expr).add(&var.v),
            OO::Float(bias) => (&self.expr).add(bias),
            OO::Int(bias) => (&self.expr).add(bias),
        }?;
        Ok(Self::new(expr))
    }

    pub fn __sub__(&self, rhs: OO) -> PyResult<Self> {
        let expr = match rhs {
            OO::Expr(expr) => (&self.expr).sub(&expr.expr),
            OO::Var(var) => (&self.expr).sub(&var.v),
            OO::Float(bias) => (&self.expr).sub(bias),
            OO::Int(bias) => (&self.expr).sub(bias),
        }?;
        Ok(Self::new(expr))
    }

    pub fn __mul__(&self, rhs: OO) -> PyResult<Self> {
        let expr = match rhs {
            OO::Expr(expr) => (&self.expr).mul(&expr.expr),
            OO::Var(var) => (&self.expr).mul(&var.v),
            OO::Float(bias) => (&self.expr).mul(bias),
            OO::Int(bias) => (&self.expr).mul(bias),
        }?;
        Ok(Self::new(expr))
    }

    pub fn __radd__(&self, lhs: OO) -> PyResult<Self> {
        self.__add__(lhs)
    }

    pub fn __rsub__(&self, lhs: OO) -> PyResult<Self> {
        self.__neg__().__add__(lhs)
    }

    pub fn __rmul__(&self, lhs: OO) -> PyResult<Self> {
        self.__mul__(lhs)
    }

    pub fn __iadd__(&mut self, rhs: OO) -> PyResult<()> {
        match rhs {
            OO::Expr(expr) => (&mut self.expr).add_assign(&expr.expr),
            OO::Var(var) => (&mut self.expr).add_assign(&var.v),
            OO::Float(bias) => (&mut self.expr).add_assign(bias),
            OO::Int(bias) => (&mut self.expr).add_assign(bias),
        }?;
        Ok(())
    }

    pub fn __isub__(&mut self, rhs: OO) -> PyResult<()> {
        match rhs {
            OO::Expr(expr) => (&mut self.expr).sub_assign(&expr.expr),
            OO::Var(var) => (&mut self.expr).sub_assign(&var.v),
            OO::Float(bias) => (&mut self.expr).sub_assign(bias),
            OO::Int(bias) => (&mut self.expr).sub_assign(bias),
        }?;
        Ok(())
    }

    pub fn __imul__(&mut self, rhs: OO) -> PyResult<()> {
        match rhs {
            OO::Expr(expr) => (&mut self.expr).mul_assign(&expr.expr),
            OO::Var(var) => (&mut self.expr).mul_assign(&var.v),
            OO::Float(bias) => (&mut self.expr).mul_assign(bias),
            OO::Int(bias) => (&mut self.expr).mul_assign(bias),
        }?;
        Ok(())
    }

    pub fn __pow__(&mut self, other: usize, modulo: Option<isize>) -> PyResult<Self> {
        if modulo.is_some() {
            return Err(PyLunaModelError::new_err(
                "the 'modulo' parameter is not supported.",
            ));
        }
        Ok(Self::new(self.expr.pow(other)?))
    }

    pub fn __ipow__(&mut self, other: usize, modulo: Option<isize>) -> PyResult<()> {
        if modulo.is_some() {
            return Err(PyLunaModelError::new_err(
                "the 'modulo' parameter is not supported.",
            ));
        }
        Ok(self.expr.pow_assign(other)?)
    }

    pub fn __neg__(&self) -> Self {
        Self::new(self.expr.neg())
    }
}

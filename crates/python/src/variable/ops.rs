use lunamodel_core::ops::LmPow;
use pyo3::prelude::*;
use std::ops::{Add, Mul, Sub, Neg, Not};

use super::PyVariable;
use crate::{expression::PyExpression as PyE, utils::OpsOther as OO};

#[pymethods]
impl PyVariable {
    pub fn __add__(&self, rhs: OO) -> PyResult<PyE> {
        self.v.check_living()?;
        match rhs {
            OO::Expr(expr) => expr.__add__(OO::Var(self.clone())),
            OO::Var(var) => Ok(PyE::new((&self.v).add(&var.v)?)),
            OO::Float(bias) => Ok(PyE::new((&self.v).add(bias)?)),
            OO::Int(bias) => Ok(PyE::new((&self.v).add(bias)?)),
        }
    }

    pub fn __sub__(&self, rhs: OO) -> PyResult<PyE> {
        self.v.check_living()?;
        match rhs {
            OO::Expr(expr) => expr.__rsub__(OO::Var(self.clone())),
            OO::Var(var) => Ok(PyE::new((&self.v).sub(&var.v)?)),
            OO::Float(bias) => Ok(PyE::new((&self.v).sub(bias)?)),
            OO::Int(bias) => Ok(PyE::new((&self.v).sub(bias)?)),
        }
    }

    pub fn __mul__(&self, rhs: OO) -> PyResult<PyE> {
        self.v.check_living()?;
        match rhs {
            OO::Expr(expr) => expr.__mul__(OO::Var(self.clone())),
            OO::Var(var) => Ok(PyE::new((&self.v).mul(&var.v)?)),
            OO::Float(bias) => Ok(PyE::new((&self.v).mul(bias)?)),
            OO::Int(bias) => Ok(PyE::new((&self.v).mul(bias)?)),
        }
    }

    pub fn __radd__(&self, lhs: OO) -> PyResult<PyE> {
        self.__add__(lhs)
    }

    pub fn __rsub__(&self, lhs: OO) -> PyResult<PyE> {
        self.__neg__()?.__add__(lhs)
    }

    pub fn __rmul__(&self, lhs: OO) -> PyResult<PyE> {
        self.__mul__(lhs)
    }

    pub fn __invert__(&self) -> PyResult<Self> {
        self.v.check_living()?;
        Ok(Self::new(self.v.not()?))
    }

    pub fn __pow__(&self, val: usize, _m: Option<isize>) -> PyResult<PyE> {
        self.v.check_living()?;
        Ok(PyE::new(self.v.pow(val)?))
    }

    pub fn __neg__(&self) -> PyResult<PyE> {
        self.v.check_living()?;
        Ok(PyE::new(self.v.neg()))
    }
}

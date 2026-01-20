use lunamodel_core::ops::LmPow;
use lunamodel_error::py::PyLunaModelError;
use pyo3::prelude::*;
use std::ops::{Add, Mul, Neg, Not, Sub};

use super::PyVariable;
use crate::{
    expression::PyExpression as PyE,
    utils::{OpsOther as OO, as_usize, as_usize_from_pyany},
};

#[pymethods]
impl PyVariable {
    pub fn __add__(&self, rhs: OO) -> PyResult<PyE> {
        self.v.check_living()?;
        match rhs {
            OO::Expr(expr) => expr.__add__(OO::Var(self.clone())),
            OO::Var(var) => Ok(PyE::new((&self.v).add(&var.v)?)),
            OO::Num(bias) => Ok(PyE::new((&self.v).add(bias)?)),
        }
    }

    pub fn __sub__(&self, rhs: OO) -> PyResult<PyE> {
        match rhs {
            OO::Expr(expr) => Ok(PyE::new((&self.v).sub(&expr.expr)?)),
            OO::Var(var) => Ok(PyE::new((&self.v).sub(&var.v)?)),
            OO::Num(bias) => Ok(PyE::new((&self.v).sub(bias)?)),
        }
    }

    pub fn __mul__(&self, rhs: OO) -> PyResult<PyE> {
        self.v.check_living()?;
        match rhs {
            OO::Expr(expr) => expr.__mul__(OO::Var(self.clone())),
            OO::Var(var) => Ok(PyE::new((&self.v).mul(&var.v)?)),
            OO::Num(bias) => Ok(PyE::new((&self.v).mul(bias)?)),
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

    pub fn inv(&self) -> PyResult<Self> {
        self.__invert__()
    }

    // #[pyo3(signature=(val, modulo=None))]
    pub fn __pow__(&self, py: Python, rhs: Py<PyAny>, modulo: Option<isize>) -> PyResult<PyE> {
        self.v.check_living()?;
        if modulo.is_some() {
            return Err(PyLunaModelError::new_err(
                "the 'modulo' parameter is not supported.",
            ));
        }
        Ok(PyE::new(self.v.pow(as_usize_from_pyany(py, rhs)?)?))
    }

    pub fn __neg__(&self) -> PyResult<PyE> {
        self.v.check_living()?;
        Ok(PyE::new(self.v.neg()?))
    }
}

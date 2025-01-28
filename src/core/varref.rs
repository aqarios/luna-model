use std::ops::{Add, Mul, Sub};

#[cfg(feature = "py")]
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "py")]
use pyo3::prelude::*;

use super::{
    environment::Environment, exceptions::VariableExistsException, expression::Expression,
};

pub type VarId = u32;

#[cfg_attr(feature = "py", pyclass(name = "Variable", subclass))]
#[derive(Clone)]
pub struct VarRef {
    pub id: VarId,
}

impl VarRef {
    pub fn new(id: VarId) -> Self {
        Self { id }
    }
}

impl Add<f64> for &VarRef {
    type Output = Expression;

    fn add(self, rhs: f64) -> Self::Output {
        Expression::new_with_constant(self.id, rhs)
    }
}

impl Add<VarRef> for &VarRef {
    type Output = Expression;

    fn add(self, rhs: VarRef) -> Self::Output {
        let mut expr = Expression::empty();
        expr.linear += self;
        expr.linear += rhs;
        expr
    }
}

impl Add<&VarRef> for &VarRef {
    type Output = Expression;

    fn add(self, rhs: &VarRef) -> Self::Output {
        let mut expr = Expression::empty();
        expr.linear += self;
        expr.linear += rhs;
        expr
    }
}

impl Mul<f64> for &VarRef {
    type Output = Expression;

    fn mul(self, rhs: f64) -> Self::Output {
        Expression::new_with_linear(self.id, rhs)
    }
}

impl Sub<f64> for &VarRef {
    type Output = Expression;

    fn sub(self, rhs: f64) -> Self::Output {
        Expression::new_with_constant(self.id, -rhs)
    }
}

#[cfg(feature = "py")]
#[pymethods]
impl VarRef {
    #[new]
    #[pyo3(signature=(name, environment))]
    fn py_new(name: String, environment: &mut Environment) -> PyResult<VarRef> {
        environment.add_var(&name).map_err(|e| {
            VariableExistsException::new_err(format!(
                "variable with name '{}' already exists",
                name
            ))
        })
    }

    fn name(&self, environment: &Environment) -> String {
        environment.get_var(self.id).name.clone()
    }

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(value) = &other.extract::<VarRef>(py) {
            let expr = self + value;
            Ok(expr)
        } else if let Ok(value) = other.extract::<f64>(py) {
            let expr = self + value;
            Ok(expr)
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __radd__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        self.__add__(py, other)
    }

    fn __mul__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(value) = other.extract::<f64>(py) {
            let expr = self * value;
            Ok(expr)
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __rmul__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        self.__mul__(py, other)
    }

    fn __sub__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(value) = other.extract::<f64>(py) {
            let expr = self - value;
            Ok(expr)
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }
}

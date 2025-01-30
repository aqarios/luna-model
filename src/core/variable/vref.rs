use std::ops::{Add, Mul};

#[cfg(feature = "py")]
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::{
    environment::EnvId,
    exceptions::{
        DifferentEnvsError, VariableExistsException, VariablesFromDifferentEnvsError,
        VariablesFromDifferentEnvsException,
    },
    expression::Expression,
    term::{Constant, Linear},
    Environment,
};

use super::{Bounds, Vtype};

pub type VarId = u32;

#[cfg_attr(feature = "py", pyclass(name = "Variable", subclass))]
#[derive(Clone)]
pub struct VarRef {
    pub id: VarId,
    pub env_id: EnvId,
}

impl VarRef {
    pub fn new(id: VarId, env_id: EnvId) -> Self {
        Self { id, env_id }
    }
}

impl Add<&VarRef> for &VarRef {
    type Output = Result<Expression, VariablesFromDifferentEnvsError>;

    fn add(self, rhs: &VarRef) -> Self::Output {
        let linear = Linear::new_from_vars((self, 1.0), (rhs, 1.0))?;
        Ok(Expression::new_from_linear(linear))
    }
}

impl Add<&f64> for &VarRef {
    type Output = Expression;

    fn add(self, rhs: &f64) -> Self::Output {
        Expression::new_from_linear_with_constant(Linear::new((self, 1.0)), Constant::new(*rhs))
    }
}

impl Mul<&f64> for &VarRef {
    type Output = Expression;

    fn mul(self, rhs: &f64) -> Self::Output {
        Expression::new_from_linear(Linear::new((self, *rhs)))
    }
}

#[cfg(feature = "py")]
#[pymethods]
impl VarRef {
    #[new]
    #[pyo3(signature=(name, environment, vtype=None, bounds=None))]
    fn py_new(
        name: String,
        environment: &mut Environment,
        vtype: Option<Vtype>,
        bounds: Option<Bounds>,
    ) -> PyResult<VarRef> {
        environment
            .add_var(&name, vtype, bounds)
            .map_err(|e| VariableExistsException::new_err(format!("{}: {}", e.to_string(), name)))
    }

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(v) = &other.extract::<VarRef>(py) {
            self.add(v)
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else if let Ok(v) = &other.extract::<f64>(py) {
            Ok(self.add(v))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn __radd__(&self, other: f64) -> Expression {
        self.add(&other)
    }

    fn __mul__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(v) = &other.extract::<f64>(py) {
            Ok(self.mul(v))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn __rmul__(&self, other: f64) -> Expression {
        self.mul(&other)
    }
}

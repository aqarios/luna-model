use std::cell::RefCell;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

use super::py_constr::PyConstraint;
use super::py_env::{PyEnvironment, CURRENT_ENV};
use super::py_exceptions::NoActiveEnvironmentFoundError;
use super::{py_bounds::PyBounds, py_expr::PyExpression};
use crate::core::expression::ExpressionBaseCreation;
use crate::core::operations::{
    AddToExpression, MulAssignToExpression, MulToExpression, NegToExpression, RSubToExpression,
    SubAssignToExpression, SubToExpression,
};
use crate::core::{
    environment, Comparator, ConcreteConstraint, ConcreteExpression, ConcreteRcVarRef,
    ConcreteVarRef, Expression, Vtype,
};
use derive_more::{Deref, DerefMut};
use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;

#[pyclass(unsendable, subclass, name = "Variable", module = "aqmodels")]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyVariable(pub ConcreteRcVarRef);

impl PyVariable {
    pub fn new(varref: ConcreteVarRef) -> Self {
        Self(varref.into())
    }
}

#[pymethods]
impl PyVariable {
    #[new]
    #[pyo3(signature=(name, env=None, vtype=None, bounds=None))]
    fn py_new(
        name: String,
        env: Option<&mut PyEnvironment>,
        vtype: Option<Vtype>,
        bounds: Option<PyBounds>,
    ) -> PyResult<Self> {
        let env: PyEnvironment = match env {
            Some(env) => env.clone(),
            None => CURRENT_ENV.with(|current| {
                current.borrow().clone().ok_or_else(|| {
                    NoActiveEnvironmentFoundError::new_err("no active environment found.")
                })
            })?,
        };

        Ok(PyVariable::new(environment::add_variable(
            env.into(),
            &name,
            vtype.as_ref(),
            bounds.map(|pb| pb.into()),
        )?))
    }

    #[getter(name)]
    fn name(&self) -> String {
        let idx: usize = self.id.into();
        let name = &self.env.borrow().variables[idx].name;
        name.clone()
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.name().hash(&mut s);
        s.finish()
    }

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.add(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.add(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = rhs.borrow().add(self.as_ref())?;
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }
        Ok(PyExpression::new(expr))
    }

    fn __radd__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        self.__add__(py, other)
    }

    fn __sub__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.add(-rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.sub(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = rhs.borrow().mul(-1.0).add(self.as_ref())?;
            // rhs.borrow()
            //     .add(self.as_ref())
            //     .map(|e| PyExpression::new(e))
            //     .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }

        Ok(PyExpression::new(expr))
    }

    fn __rsub__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(PyExpression::new(self.rsub(rhs)))
        } else {
            Err(PyTypeError::new_err("unsupported type for operation"))
        }
    }

    fn __mul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.mul(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.mul(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = rhs.borrow().mul(self.as_ref())?;
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }

        Ok(PyExpression::new(expr))
    }

    fn __rmul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        self.__mul__(py, other)
    }

    fn __pow__(&self, other: usize, modparam: Option<usize>) -> PyResult<PyExpression> {
        if modparam.is_some() {
            return Err(PyRuntimeError::new_err(
                "the parameter 'mod' is not supported.",
            ));
        }
        let expr = match other {
            0 => Expression::empty(Rc::clone(&self.env)).add(1.0),
            1 => Expression::new_linear_single(Rc::clone(&self.env), self.id, 1.0),
            2 => Expression::new_quadratic(Rc::clone(&self.env), self.id, self.id, 1.0),
            _ => {
                let mut base = Expression::new_linear_single(Rc::clone(&self.env), self.id, 1.0);
                for _ in 1..other {
                    base.mul_assign(self.as_ref())?;
                }
                base
            }
        };
        Ok(PyExpression::new(expr))
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    fn __neg__(&self) -> PyExpression {
        PyExpression::new(self.0.neg())
    }

    fn __eq__(&self, py: Python, rhs: PyObject) -> PyResult<PyConstraint> {
        self.make_constraint(py, rhs, Comparator::Eq)
    }

    fn __le__(&self, py: Python, rhs: PyObject) -> PyResult<PyConstraint> {
        self.make_constraint(py, rhs, Comparator::Le)
    }

    fn __ge__(&self, py: Python, rhs: PyObject) -> PyResult<PyConstraint> {
        self.make_constraint(py, rhs, Comparator::Ge)
    }
}

impl PyVariable {
    fn make_constraint(
        &self,
        py: Python,
        rhs: PyObject,
        comparator: Comparator,
    ) -> PyResult<PyConstraint> {
        let mut lhs = Expression::new_linear_single(Rc::clone(&self.env), self.id, 1.0);
        let bias: PyResult<f64> = if let Ok(bias) = rhs.extract::<f64>(py) {
            Ok(bias)
        } else if let Ok(var) = rhs.extract::<PyVariable>(py) {
            lhs.sub_assign(var.as_ref())?;
            Ok(0.0)
        } else if let Ok(expr) = rhs.extract::<PyExpression>(py) {
            lhs.sub_assign(expr.borrow().deref())?;
            Ok(0.0)
        } else {
            Err(PyTypeError::new_err("unsupported type for operation"))
        };
        Ok(PyConstraint::new(ConcreteConstraint::new(
            Rc::new(RefCell::new(lhs)),
            bias?,
            comparator,
            None,
        )?))
    }
}

#[pymethods]
impl Vtype {
    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

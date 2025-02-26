use super::{py_var::PyVariable, types::Expr};
use crate::core::{
    operations::{AddAssignToExpression, AddToExpression, MulAssignToExpression, MulToExpression},
    ExpressionBase, VariablesFromDifferentEnvsException,
};
use derive_more::{Deref, DerefMut};
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use std::{cell::RefCell, rc::Rc};

#[pyclass(unsendable, name = "Expression")]
#[derive(Deref, DerefMut, Clone)]
pub struct PyExpression(pub Rc<RefCell<Expr>>);

impl PyExpression {
    pub fn new(expression: Expr) -> Self {
        Self(Rc::new(RefCell::new(expression)))
    }
}

#[pymethods]
impl PyExpression {
    fn get_linear(&self, var: &PyVariable) -> PyResult<f64> {
        Ok(self.borrow().linear(var.id)?)
    }

    fn get_offset(&self) -> f64 {
        self.borrow().offset()
    }

    fn get_quadratic(&self, u: &PyVariable, v: &PyVariable) -> PyResult<f64> {
        Ok(self.borrow().quadratic(u.id, v.id)?)
    }

    fn get_higher_order(&self, vars: Vec<PyVariable>) -> PyResult<f64> {
        // todo: optimize the iter away...
        Ok(self
            .borrow()
            .higher_order(&vars.iter().map(|v| v.id).collect())?)
    }

    #[pyo3(name = "num_variables")]
    fn get_num_variables(&self) -> usize {
        self.borrow().num_variables()
    }

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(PyExpression::new(self.borrow().add(rhs)))
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            self.borrow()
                .add(rhs.as_ref())
                .map(|e| PyExpression::new(e))
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            self.borrow()
                .add(rhs.borrow())
                .map(|e| PyExpression::new(e))
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }
    fn __radd__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }
    fn __sub__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }
    fn __rsub__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }
    fn __mul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(PyExpression::new(self.borrow().mul(rhs)))
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            self.borrow()
                .mul(rhs.as_ref())
                .map(|e| PyExpression::new(e))
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            self.borrow()
                .mul(rhs.borrow())
                .map(|e| PyExpression::new(e))
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }
    fn __rmul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }
    // In place assignment
    fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(self.borrow_mut().add_assign(rhs))
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            self.borrow_mut()
                .add_assign(rhs.as_ref())
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            self.borrow_mut()
                .add_assign(rhs.borrow())
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }
    fn __isub__(&mut self, py: Python, other: PyObject) {
        todo!()
    }
    fn __imul__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(self.borrow_mut().mul_assign(rhs))
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            self.borrow_mut()
                .mul_assign(rhs.as_ref())
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            self.borrow_mut()
                .mul_assign(rhs.borrow())
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }
    // Unary operations
    fn __pos__(&mut self) {
        todo!()
    }
    fn __new__(&mut self) {
        todo!()
    }
    // Comparison
    fn __eq__(&self, other: &Self) -> bool {
        *self.borrow() == *other.borrow()
    }

    fn __ne__(&self, other: &Self) -> bool {
        *self.borrow() == *other.borrow()
    }
}

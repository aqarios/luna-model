use std::{cell::RefCell, ops::AddAssign, rc::Rc};

use derive_more::{Deref, DerefMut};
use pyo3::{exceptions::PyRuntimeError, prelude::*};

use crate::core::Comparator;

use super::{
    py_expr::PyExpression,
    types::{Constr, Constrs},
};

#[pyclass(unsendable, name = "Constraints")]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyConstraints(pub Rc<RefCell<Constrs>>);

impl PyConstraints {
    pub fn new(constrs: Constrs) -> Self {
        Self(Rc::new(RefCell::new(constrs)))
    }
}

#[pyclass(unsendable, name = "Constraint")]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyConstraint(pub Rc<RefCell<Constr>>);

impl PyConstraint {
    pub fn new(constraint: Constr) -> Self {
        Self(Rc::new(RefCell::new(constraint)))
    }

    pub fn new_py(
        expr: &PyExpression,
        py: Python,
        other: PyObject,
        comparator: Comparator,
    ) -> PyResult<PyConstraint> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(PyConstraint::new(Constr::new(
                Rc::clone(&expr.0),
                rhs,
                comparator,
            )))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }
}

#[pymethods]
impl PyConstraint {
    fn __eq__(&self, other: Self) -> bool {
        *self.borrow() == *other.borrow()
    }
}

#[pymethods]
impl PyConstraints {
    fn __iadd__(&mut self, other: PyConstraint) {
        self.add_constraint(other);
    }

    fn add_constraint(&mut self, other: PyConstraint) {
        self.borrow_mut().add_assign(other.borrow());
    }

    fn __eq__(&self, other: Self) -> bool {
        *self.borrow() == *other.borrow()
    }
}

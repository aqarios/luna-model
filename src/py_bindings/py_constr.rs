use std::{cell::RefCell, rc::Rc};

use pyo3::{exceptions::PyRuntimeError, prelude::*};

use crate::core::Comparator;

use super::{py_expr::PyExpression, types::Constr};

#[pyclass(unsendable, name = "Constraint")]
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
impl PyConstraint {}

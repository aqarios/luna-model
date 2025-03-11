use std::{cell::RefCell, ops::AddAssign, rc::Rc};

use derive_more::{Deref, DerefMut};
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyBytes};

use crate::{
    core::Comparator,
    serialization_v2::{decode_constraints, encode_constraints},
};

use super::{
    py_env::PyEnvironment,
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

    fn __str__(&self) -> String {
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.borrow())
    }
}

#[pymethods]
impl PyConstraints {
    #[new]
    fn py_new() -> Self {
        PyConstraints::new(Constrs::default())
    }

    fn __iadd__(&mut self, other: PyConstraint) {
        self.add_constraint(other);
    }

    fn add_constraint(&mut self, other: PyConstraint) {
        self.borrow_mut().add_assign(other.borrow());
    }

    fn __eq__(&self, other: Self) -> bool {
        *self.borrow() == *other.borrow()
    }

    fn __str__(&self) -> String {
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.borrow())
    }

    #[pyo3(signature=(compress=None, level=None))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<PyObject> {
        Ok(PyBytes::new(
            py,
            &encode_constraints(&self.borrow(), compress.unwrap_or(level.is_some()), level)?,
        )
        .into())
    }

    #[pyo3(signature=(compress=None, level=None))]
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        self.serialize(py, compress, level)
    }

    #[staticmethod]
    fn deserialize(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        // todo, handle env
        let bytes: &[u8] = data.as_bytes(py);
        let constrs = decode_constraints(bytes, Rc::clone(&PyEnvironment::new().0));
        match constrs {
            Ok(expr) => Ok(PyConstraints::new(expr)),
            Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    fn decode(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::deserialize(py, data)
    }
}

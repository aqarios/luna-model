use std::{
    cell::RefCell,
    ops::{AddAssign, Deref},
    rc::Rc,
};

use derive_more::{Deref, DerefMut};
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyBytes};

use crate::{
    core::{
        expression::ExpressionBaseCreation, Comparator, ConcreteConstraint, ConcreteConstraints,
        ConcreteMutRcConstraint, ConcreteMutRcConstraints, Constraint, Create, Expression,
    },
    serialization::{
        Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
    },
};

use super::{py_env::PyEnvironment, py_expr::PyExpression, py_var::PyVariable};

#[pyclass(unsendable, name = "Constraints", module = "aqmodels")]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyConstraints(pub ConcreteMutRcConstraints);

impl PyConstraints {
    pub fn new(constrs: ConcreteConstraints) -> Self {
        Self(constrs.into())
    }
}

#[pyclass(unsendable, name = "Constraint", module = "aqmodels")]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyConstraint(pub ConcreteMutRcConstraint);

impl PyConstraint {
    pub fn new(constraint: ConcreteConstraint) -> Self {
        Self(constraint.into())
    }

    pub fn new_py(
        py: Python,
        expr: &PyExpression,
        other: PyObject,
        comparator: Comparator,
    ) -> PyResult<PyConstraint> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(PyConstraint::new(Constraint::new(
                Rc::clone(&expr.0),
                rhs,
                comparator,
                None,
            )?))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }
}

#[pymethods]
impl PyConstraint {
    #[new]
    #[pyo3(signature=(lhs, rhs, comparator, name=None))]
    fn py_new(
        py: Python,
        lhs: PyObject,
        rhs: f64,
        comparator: Comparator,
        name: Option<String>,
    ) -> PyResult<Self> {
        if let Ok(expr) = lhs.extract::<PyExpression>(py) {
            Ok(PyConstraint::new(Constraint::new(
                expr.0, rhs, comparator, name,
            )?))
        } else if let Ok(var) = lhs.extract::<PyVariable>(py) {
            let expr = Expression::new_linear_single(Rc::clone(&var.env), var.id, 1.0);
            Ok(PyConstraint::new(ConcreteConstraint::new(
                Rc::new(RefCell::new(expr)),
                rhs,
                comparator,
                name,
            )?))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn __eq__(&self, other: Self) -> bool {
        *self.borrow() == *other.borrow()
    }

    fn __str__(&self) -> String {
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.borrow())
    }

    #[getter]
    fn name(&self) -> Option<String> {
        self.borrow().name.clone()
    }
}

#[pymethods]
impl PyConstraints {
    #[new]
    fn py_new() -> Self {
        PyConstraints(ConcreteMutRcConstraints::create())
    }

    fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok((constr, name)) = other.extract::<(PyConstraint, String)>(py) {
            Ok(self.add_constraint(constr, Some(name))?)
        } else if let Ok(constr) = other.extract::<PyConstraint>(py) {
            Ok(self.add_constraint(constr, None)?)
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    #[pyo3(signature=(constraint, name=None))]
    fn add_constraint(&mut self, constraint: PyConstraint, name: Option<String>) -> PyResult<()> {
        constraint.borrow_mut().set_name(name)?;
        self.borrow_mut().add_assign(constraint.borrow().deref());
        Ok(())
    }

    fn __getitem__(&self, n: usize) -> PyResult<PyConstraint> {
        // todo: can we remove the clone here? acceptable for now. Make it more like
        // a view.
        Ok(PyConstraint::new(self.borrow().get_constraint(n)?.clone()))
    }

    fn __eq__(&self, other: Self) -> bool {
        *self.borrow() == *other.borrow()
    }

    fn __str__(&self) -> String {
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.borrow())
    }

    #[pyo3(signature=(compress=None, level=None))]
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        let compress = compress.unwrap_or(level.is_some());
        Ok(PyBytes::new(
            py,
            &self
                .borrow()
                .deref()
                .encode()
                .maybe_compress(compress, level)?
                .versionize(),
        )
            .into())
    }

    #[pyo3(signature=(compress=None, level=None))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<PyObject> {
        self.encode(py, compress, level)
    }

    #[staticmethod]
    fn decode(py: Python, data: Py<PyBytes>, env: PyEnvironment) -> PyResult<Self> {
        Ok(PyConstraints::new(
            data.as_bytes(py)
                .unversionize()
                .decompress()?
                .decode(env.0)?,
        ))
    }

    #[staticmethod]
    fn deserialize(py: Python, data: Py<PyBytes>, env: PyEnvironment) -> PyResult<Self> {
        Self::decode(py, data, env)
    }
}

#[pymethods]
impl Comparator {
    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

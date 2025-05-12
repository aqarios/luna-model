use std::{
    cell::RefCell,
    ops::{AddAssign, Deref},
    rc::Rc,
};

use derive_more::{Deref, DerefMut};
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyBytes};

use crate::{
    core::{
        expression::ExpressionBaseCreation, operations::SubAssignToExpression, Comparator, ConcreteConstraint, ConcreteConstraints, ConcreteExpression, ConcreteMutRcConstraint, ConcreteMutRcConstraints, Constraint, Create, Expression
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
        lhs: &PyExpression,
        rhs: PyObject,
        comparator: Comparator,
    ) -> PyResult<PyConstraint> {
        let mut lhs = lhs.borrow().deref().clone();
        let bias: PyResult<f64> = if let Ok(bias) = rhs.extract::<f64>(py) {
            Ok(bias)
        } else if let Ok(var) = rhs.extract::<PyVariable>(py) {
            lhs.sub_assign(var.as_ref())?;
            Ok(0.0)
        } else if let Ok(expr) = rhs.extract::<PyExpression>(py) {
            lhs.sub_assign(expr.borrow().deref())?;
            Ok(0.0)
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        };
        Ok(PyConstraint::new(Constraint::new(
            Rc::new(RefCell::new(lhs)),
            bias?,
            comparator,
            None,
        )?))
    }
}

#[pymethods]
impl PyConstraint {
    #[new]
    #[pyo3(signature=(lhs, rhs, comparator, name=None))]
    fn py_new(
        py: Python,
        lhs: PyObject,
        rhs: PyObject,
        comparator: Comparator,
        name: Option<String>,
    ) -> PyResult<Self> {
        let lhs: PyResult<ConcreteExpression> = if let Ok(expr) = lhs.extract::<PyExpression>(py) {
            Ok(expr.0.borrow().deref().clone())
        } else if let Ok(var) = lhs.extract::<PyVariable>(py) {
            Ok(Expression::new_linear_single(Rc::clone(&var.env), var.id, 1.0))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for lhs in operation"))
        };
        let mut lhs = lhs?;
        let bias = if let Ok(bias) = rhs.extract::<f64>(py) {
            Ok(bias)
        } else if let Ok(var) = rhs.extract::<PyVariable>(py) {
            lhs.sub_assign(var.as_ref())?;
            Ok(0.0)
        } else if let Ok(expr) = rhs.extract::<PyExpression>(py) {
            lhs.sub_assign(expr.borrow().deref())?;
            Ok(0.0)
        } else {
            Err(PyRuntimeError::new_err("unsopported type for rhs in operation"))
        };
        Ok(PyConstraint::new(ConcreteConstraint::new(
            Rc::new(RefCell::new(lhs)),
            bias?,
            comparator,
            name,
        )?))
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

    #[getter]
    fn lhs(&self) -> PyExpression {
        PyExpression(Rc::clone(&self.borrow().lhs))
    }

    #[getter]
    fn rhs(&self) -> f64 {
        self.borrow().rhs
    }

    #[getter]
    fn comparator(&self) -> Comparator {
        self.borrow().comparator
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

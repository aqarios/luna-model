use super::{
    py_constr::PyConstraint,
    py_env::{PyEnvironment, CURRENT_ENV},
    py_exceptions::NoActiveEnvironmentFoundException,
    py_var::PyVariable,
};
use crate::core::{
    operations::{AddAssignToExpression, AddToExpression, MulAssignToExpression, MulToExpression},
    Comparator, ConcreteConstraint, ConcreteExpression, ConcreteMutRcExpression, Expression,
    ExpressionBase,
};
use crate::{
    core::expression::ExpressionBaseCreation,
    serialization::{
        Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
    },
};
use derive_more::{Deref, DerefMut};
use pyo3::{
    exceptions::PyRuntimeError,
    prelude::*,
    types::{PyBool, PyBytes},
};
use std::{ops::Deref, rc::Rc};

#[pyclass(unsendable, name = "Expression", module = "aqmodels")]
#[derive(Deref, DerefMut, Clone)]
pub struct PyExpression(pub ConcreteMutRcExpression);

impl PyExpression {
    pub fn new(expression: ConcreteExpression) -> Self {
        Self(expression.into())
    }
}

#[pymethods]
impl PyExpression {
    #[new]
    #[pyo3(signature=(env=None))]
    fn py_new(env: Option<&mut PyEnvironment>) -> PyResult<Self> {
        let env: PyEnvironment = match env {
            Some(env) => env.clone(),
            None => CURRENT_ENV.with(|current| {
                current.borrow().clone().ok_or_else(|| {
                    NoActiveEnvironmentFoundException::new_err("no active environment found.")
                })
            })?,
        };
        Ok(PyExpression::new(Expression::empty(env.0)))
    }

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
        Ok(PyExpression::new(
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

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.borrow().add(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.borrow().add(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = self.borrow().add(rhs.borrow().deref())?;
        } else {
            return Err(PyRuntimeError::new_err("unsopported type for operation"));
        }

        Ok(PyExpression::new(expr))
    }

    fn __radd__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        self.__add__(py, other)
    }

    fn __sub__(&self, _py: Python, _other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }

    fn __rsub__(&self, _py: Python, _other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }

    fn __mul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.borrow().mul(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.borrow().mul(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = self.borrow().mul(rhs.borrow().deref())?;
        } else {
            return Err(PyRuntimeError::new_err("unsopported type for operation"));
        }
        Ok(PyExpression::new(expr))
    }

    fn __rmul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        self.__mul__(py, other)
    }

    // In place assignment
    fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            self.borrow_mut().add_assign(rhs)
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            self.borrow_mut().add_assign(rhs.as_ref())?
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            self.borrow_mut().add_assign(rhs.borrow().deref())?
        } else {
            return Err(PyRuntimeError::new_err("unsopported type for operation"));
        }

        Ok(())
    }

    fn __isub__(&mut self, _py: Python, _other: PyObject) {
        todo!()
    }

    fn __imul__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            self.borrow_mut().mul_assign(rhs)
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            self.borrow_mut().mul_assign(rhs.as_ref())?
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            self.borrow_mut().mul_assign(rhs.borrow().deref())?
        } else {
            return Err(PyRuntimeError::new_err("unsopported type for operation"));
        }
        Ok(())
    }

    // Unary operations
    // fn __pos__(&mut self) {
    //     todo!()
    // }

    // fn __new__(&mut self) {
    //     todo!()
    // }

    // Comparison
    fn __eq__(&self, py: Python, other: PyObject) -> PyResult<PyObject> {
        if let Ok(rhs) = other.extract::<PyExpression>(py) {
            // Actual equality check.
            let result = *self.borrow() == *rhs.borrow();
            Ok(PyBool::new(py, result).to_owned().into())
        } else if let Ok(rhs) = other.extract::<f64>(py) {
            // Creates a new constraint.
            let constraint = ConcreteConstraint::new(Rc::clone(&self.0), rhs, Comparator::Eq);
            // todo: this is depreated... change to the new way
            // but for now this works as intended
            Ok(PyConstraint::new(constraint).into_py(py))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn __le__(&self, py: Python, other: PyObject) -> PyResult<PyConstraint> {
        PyConstraint::new_py(py, &self, other, Comparator::Leq)
    }

    fn __ge__(&self, py: Python, other: PyObject) -> PyResult<PyConstraint> {
        PyConstraint::new_py(py, &self, other, Comparator::Geq)
    }

    fn __ne__(&self, other: &Self) -> bool {
        *self.borrow() != *other.borrow()
    }

    fn __str__(&self) -> String {
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.borrow())
    }
}

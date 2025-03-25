use super::py_env::{PyEnvironment, CURRENT_ENV};
use super::py_exceptions::NoActiveEnvironmentFoundError;
use super::{py_bounds::PyBounds, py_expr::PyExpression};
use crate::core::operations::{
    AddToExpression, MulToExpression, RSubToExpression, SubToExpression,
};
use crate::core::{environment, ConcreteExpression, ConcreteRcVarRef, ConcreteVarRef, Vtype};
use derive_more::{Deref, DerefMut};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyclass(unsendable, subclass, name = "Variable", module = "aqmodels")]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyVariable(pub ConcreteRcVarRef);

impl PyVariable {
    fn new(varref: ConcreteVarRef) -> Self {
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

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.add(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.add(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = rhs.borrow().add(self.as_ref())?;
        } else {
            return Err(PyRuntimeError::new_err("unsopported type for operation"));
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
        // } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
        //     rhs.borrow()
        //         .add(self.as_ref())
        //         .map(|e| PyExpression::new(e))
        //         .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else {
            return Err(PyRuntimeError::new_err("unsopported type for operation"));
        }

        Ok(PyExpression::new(expr))
    }

    fn __rsub__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            // scalar - variable
            Ok(PyExpression::new(self.rsub(rhs)))
        // } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
        //     self.sub(rhs.as_ref())
        //         .map(|e| PyExpression::new(e))
        //         .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        // } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
        //     rhs.borrow()
        //         .add(self.as_ref())
        //         .map(|e| PyExpression::new(e))
        //         .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
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
            return Err(PyRuntimeError::new_err("unsopported type for operation"));
        }

        Ok(PyExpression::new(expr))
    }

    fn __rmul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        self.__mul__(py, other)
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pymethods]
impl Vtype {
    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

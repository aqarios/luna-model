use std::rc::Rc;

use super::py_env::{PyEnvironment, CURRENT_ENV};
use super::{py_bounds::PyBounds, py_expr::PyExpression};
use crate::core::operations::{AddToExpression, MulToExpression};
use crate::core::{
    environment, NoActiveEnvironmentFoundException, VarId, VarRef, VariableExistsException,
    VariablesFromDifferentEnvsException, Vtype,
};

use derive_more::{Deref, DerefMut};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyclass(unsendable, subclass, name = "Variable")]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyVariable(pub Rc<VarRef<VarId>>);

impl PyVariable {
    fn new(varref: VarRef<VarId>) -> Self {
        Self(Rc::new(varref))
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
                    NoActiveEnvironmentFoundException::new_err("no active environment found.")
                })
            })?,
        };

        environment::add_varibale(env.0, &name, vtype.as_ref(), bounds.as_deref())
            .map(|v| PyVariable::new(v))
            .map_err(|e| VariableExistsException::new_err(format!("{}: {}", e.to_string(), name)))
    }

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(PyExpression::new(self.add(rhs)))
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            self.add(rhs.as_ref())
                .map(|e| PyExpression::new(e))
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            rhs.borrow()
                .add(self.as_ref())
                .map(|e| PyExpression::new(e))
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn __mul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(PyExpression::new(self.mul(rhs)))
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            self.mul(rhs.as_ref())
                .map(|e| PyExpression::new(e))
                .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            todo!()
            // rhs.borrow()
            //     .mul(self.as_ref())
            //     .map(|e| PyExpression::new(e))
            //     .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }
}

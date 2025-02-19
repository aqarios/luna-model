use std::rc::Rc;

use super::py_env::{PyEnvironment, CURRENT_ENV};
use super::py_vtype::PyVtype;
use super::{py_bounds::PyBounds, py_expr::PyExpression};
use crate::core::{
    environment, Expression, ExpressionBaseInternal, NoActiveEnvironmentFoundException, VarRef,
    VariableExistsException, VariablesFromDifferentEnvsException,
};

use derive_more::{Deref, DerefMut};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyclass(unsendable, subclass, name = "Variable")]
#[derive(Deref, DerefMut, Clone)]
pub struct PyVariable(Rc<VarRef>);

#[pymethods]
impl PyVariable {
    #[new]
    #[pyo3(signature=(name, env=None, vtype=None, bounds=None))]
    fn new(
        name: String,
        env: Option<&mut PyEnvironment>,
        vtype: Option<PyVtype>,
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

        environment::add_varibale(env.0, &name, vtype.as_deref(), bounds.as_deref())
            .map(|v| PyVariable(Rc::new(v)))
            .map_err(|e| VariableExistsException::new_err(format!("{}: {}", e.to_string(), name)))
    }

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(PyExpression(Rc::new(
                Expression::new_from_weighted_variable(self.env.clone(), self.id, rhs),
            )))
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            if self.env.borrow().id != rhs.env.borrow().id {
                return Err(VariablesFromDifferentEnvsException::new_err(
                    "variables must be from the same environment",
                ));
            }
            // Somehow the variable is dropped twice... possbily due to the extract
            // call wich clones the variable and damages the logic internally.
            // We should fix the extract call. Maybe a Rc<_> can solve the problem.
            // We need the PyVar readonly so no RefCell<_> should not be needed.
            // Let's see...
            // So the duplicate drop call is gone, the Rc<_> wrapper for the PyVariable
            // fixed it. BUT we now have an issue in the delete_var of the env called
            // in to Drop implementation of the VarRef.
            //
            // Now the problem is that when a variable is removed, the variables vec
            // used for efficient access on the variables information based on the
            // index is removed, thus the vector is shortand. We should instead
            // use a dead marker to indicate that the variable is removed.
            // Thereby, we can keep the indexing as it should be.
            // We keep track of the dead variable and if a new variable is added we
            // would use the dead marker instead.
            //
            // For now, we don't use the dead marker as this has implications on
            // expressions, we also need to manage expressions, i.e., when the varid
            // is used in an expression it cannot be deleted, thus an error has to be
            // raised.
            //
            // Or something similar. We need to think of a good solution.
            // For now deletion works (just not memory efficient and no expr handling)
            Ok(PyExpression(Rc::new(
                Expression::new_linear_from_variables(self.env.clone(), self.id, rhs.id, 1.0),
            )))
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            todo!("this needs to be implemented on the expression")
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }
}

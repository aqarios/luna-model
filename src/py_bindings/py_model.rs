use std::rc::Rc;

use crate::{
    core::{Model, NoActiveEnvironmentFoundException, VarId},
    py_bindings::py_env::CURRENT_ENV,
};
use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

use super::{py_constr::PyConstraints, py_env::PyEnvironment, py_expr::PyExpression};

#[pyclass(unsendable, name = "Model", subclass)]
#[derive(Deref, DerefMut)]
pub struct PyModel(pub Model<VarId, f64>);

#[pymethods]
impl PyModel {
    #[new]
    #[pyo3(signature=(env=None, name=None))]
    fn py_new(env: Option<PyEnvironment>, name: Option<String>) -> PyResult<Self> {
        let env: PyEnvironment = match env {
            Some(env) => env.clone(),
            None => CURRENT_ENV.with(|curr| {
                curr.borrow().clone().ok_or_else(|| {
                    NoActiveEnvironmentFoundException::new_err("no active environment found.")
                })
            })?,
        };
        Ok(Self(Model::new_with_env(name, env.0)))
    }

    #[getter]
    fn get_objective(&self) -> PyExpression {
        PyExpression(self.objective.clone())
    }

    #[setter]
    fn set_objective(&mut self, other: &PyExpression) {
        self.objective = other.0.clone()
    }

    #[getter]
    fn get_constraints(&self) -> PyConstraints {
        PyConstraints(Rc::clone(&self.constraints))
    }

    #[setter]
    fn set_constraints(&mut self, other: &PyConstraints) {
        self.constraints = other.0.clone()
    }

    fn num_constraints(&self) -> usize {
        self.constraints.borrow().len()
    }

    #[getter]
    fn name(&self) -> &String {
        &self.name
    }

    #[getter]
    fn environment(&self) -> PyEnvironment {
        PyEnvironment(self.environment.clone())
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

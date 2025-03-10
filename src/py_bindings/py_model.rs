use std::rc::Rc;

use super::{py_constr::PyConstraints, py_env::PyEnvironment, py_expr::PyExpression};
use crate::core::expression::One;
use crate::{
    core::{Model, NoActiveEnvironmentFoundException, VarId},
    py_bindings::py_env::CURRENT_ENV,
    serialization::{decode_model, encode_model},
};
use derive_more::{Deref, DerefMut};
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyBytes};

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}

#[pyclass(unsendable, name = "Model", subclass)]
#[derive(Deref, DerefMut)]
pub struct PyModel(pub Model<VarId, f64>);

impl Into<Model<VarId, f64>> for PyModel {
    fn into(self) -> Model<VarId, f64> {
        self.0
    }
}

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

    fn serialize(&self, py: Python) -> PyObject {
        PyBytes::new(py, &encode_model(&self.0)).into()
    }

    /// Alias for serialize
    fn encode(&self, py: Python) -> PyObject {
        self.serialize(py)
    }

    #[staticmethod]
    fn deserialize(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        let bytes: &[u8] = data.as_bytes(py);
        let model_res = decode_model(bytes);
        match model_res {
            Ok(model) => Ok(PyModel(model)),
            Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    fn decode(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::deserialize(py, data)
    }
}

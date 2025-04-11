use std::rc::Rc;

use super::{
    py_constr::PyConstraints, py_env::PyEnvironment, py_expr::PyExpression, py_sol::PySolution,
};
use crate::core::RcSolution;
use crate::py_bindings::py_res::PyOwnedResult;
use crate::py_bindings::py_sample::PySample;
use crate::{
    core::{ConcreteModel, Environment, Model},
    py_bindings::py_env::CURRENT_ENV,
    serialization::{
        Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
    },
};
use derive_more::{Deref, DerefMut};
use pyo3::{prelude::*, types::PyBytes};

#[pyclass(unsendable, subclass, name = "Model", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PyModel(pub ConcreteModel);

impl Into<ConcreteModel> for PyModel {
    fn into(self) -> ConcreteModel {
        self.0
    }
}

#[pymethods]
impl PyModel {
    #[new]
    #[pyo3(signature=(env=None, name=None))]
    fn py_new(env: Option<PyEnvironment>, name: Option<String>) -> Self {
        let env: PyEnvironment = match env {
            Some(env) => env.clone(),
            None => CURRENT_ENV.with(|curr| {
                curr.borrow()
                    .clone()
                    .unwrap_or_else(|| PyEnvironment::new(Environment::new()))
            }),
            // If it show throw an error. But probably shouldn't so we create a new
            // env if not in the context.
            // None => CURRENT_ENV.with(|curr| {
            //     curr.borrow().clone().ok_or_else(|| {
            //         NoActiveEnvironmentFoundError::new_err("no active environment found.")
            //     })
            // })?,
        };
        Self(Model::new_with_env(name, env.0))
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

    #[pyo3(signature=(compress=None, level=None))]
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        let compress = compress.unwrap_or(level.is_some());
        Ok(PyBytes::new(
            py,
            &self
                .0
                .encode()
                .maybe_compress(compress, level)?
                .versionize(),
        )
            .into())
    }

    /// Alias for serialize
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
    fn decode(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Ok(PyModel(
            data.as_bytes(py).unversionize().decompress()?.decode(())?,
        ))
    }

    #[staticmethod]
    fn deserialize(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::decode(py, data)
    }

    fn evaluate(&self, solution: &PySolution) -> PySolution {
        PySolution(self.evaluate_solution(RcSolution::clone(&solution.0)))
    }

    fn evaluate_sample(&self, sample: &PySample) -> PyOwnedResult {
        PyOwnedResult(self.0.evaluate_sample(&sample.0))
    }
}

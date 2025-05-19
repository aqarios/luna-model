use std::cell::RefCell;
use std::ops::{AddAssign, Deref};
use std::rc::Rc;

use super::py_constr::PyConstraint;
use super::py_model_metadata::PyModelMetadata;
use super::{
    py_constr::PyConstraints, py_env::PyEnvironment, py_expr::PyExpression, py_sol::PySolution,
};
use crate::core::operations::AddAssignToExpression;
use crate::core::{ConcreteModel, ConcreteMutRcModel, RcSolution, Sense, VarRef};
use crate::py_bindings::py_res::PyOwnedResult;
use crate::py_bindings::py_sample::PySample;
use crate::py_bindings::py_var::PyVariable;
use crate::{
    core::{Environment, Model},
    py_bindings::py_env::CURRENT_ENV,
    serialization::{
        Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
    },
};
use derive_more::{Deref, DerefMut};
use pyo3::{prelude::*, types::PyBytes};

#[pyclass(unsendable, subclass, name = "Model", module = "aqmodels")]
#[derive(Clone, Deref, DerefMut)]
pub struct PyModel {
    #[deref]
    #[deref_mut]
    pub concrete_model: ConcreteMutRcModel,
    #[deref(ignore)]
    #[deref_mut(ignore)]
    #[pyo3(get, set)]
    pub _metadata: PyModelMetadata, // HashMap<String, PyObject>, // pub metadata: Option<PyDict>,
}

impl PyModel {
    pub fn new(model: ConcreteModel) -> Self {
        Self {
            concrete_model: Rc::new(RefCell::new(model)),
            _metadata: PyModelMetadata::new(),
        }
    }
}

impl Into<ConcreteMutRcModel> for PyModel {
    fn into(self) -> ConcreteMutRcModel {
        self.concrete_model
    }
}

#[pymethods]
impl PyModel {
    #[new]
    #[pyo3(signature=(name=None, env=None))]
    fn py_new(name: Option<String>, env: Option<PyEnvironment>) -> Self {
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
        Self::new(Model::new_with_env(name, env.0))
    }

    #[pyo3(name = "set_sense")]
    fn set_sense_py(&mut self, sense: Sense) {
        self.borrow_mut().set_sense(sense);
    }

    // #[getter]
    // #[pyo3(name = "_metadata")]
    // fn get_metadata(&self) -> PyModelMetadata {
    //     self.metadata.clone()
    // }

    // #[setter]
    // #[pyo3(name = "_metadata")]
    // fn set_metadata(&mut self, value: &PyModelMetadata) {
    //     self.metadata = value.clone()
    // }

    #[getter]
    fn get_sense(&self) -> Sense {
        self.borrow().sense
    }

    #[getter]
    fn get_objective(&self) -> PyExpression {
        PyExpression(self.borrow().objective.clone())
    }

    #[setter]
    fn set_objective(&mut self, value: &PyExpression) {
        self.borrow_mut().objective = value.0.clone()
    }

    #[getter]
    fn get_constraints(&self) -> PyConstraints {
        PyConstraints(Rc::clone(&self.borrow().constraints))
    }

    #[setter]
    fn set_constraints(&mut self, value: &PyConstraints) {
        self.borrow_mut().constraints = value.0.clone()
    }

    #[pyo3(signature=(constraint, name=None))]
    fn add_constraint(&mut self, constraint: PyConstraint, name: Option<String>) -> PyResult<()> {
        constraint.borrow_mut().set_name(name)?;
        self.borrow()
            .constraints
            .borrow_mut()
            .add_assign(constraint.borrow().deref());
        Ok(())
    }

    #[pyo3(name = "set_objective", signature=(expression, sense=None))]
    fn set_objective_direct(&mut self, expression: PyExpression, sense: Option<Sense>) -> () {
        self.borrow_mut().set_sense(sense.unwrap_or(Sense::Min));
        self.borrow_mut().objective = expression.0.clone();
    }

    fn add_objective(&mut self, expression: PyExpression) -> PyResult<()> {
        Ok(self
            .borrow()
            .objective
            .borrow_mut()
            .add_assign(expression.borrow().deref())?)
    }

    #[getter]
    fn num_constraints(&self) -> usize {
        self.borrow().constraints.borrow().len()
    }

    #[getter]
    fn name(&self) -> String {
        self.borrow().name.clone()
    }

    #[getter]
    fn environment(&self) -> PyEnvironment {
        PyEnvironment(self.borrow().environment.clone())
    }

    #[pyo3(signature=(active=None))]
    fn variables(&self, active: Option<bool>) -> Vec<PyVariable> {
        let model = self.borrow();
        let active_vars = &model.objective.borrow().active;
        (0..self.borrow().environment.borrow().varcount.into())
            .enumerate()
            .filter(|(_, a)| *active_vars.get(*a).unwrap_or(&false) || !active.unwrap_or_default())
            .map(|(i, _)| {
                PyVariable::new(VarRef {
                    id: i.into(),
                    env: Rc::clone(&model.environment),
                })
            })
            .collect()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.concrete_model == other.concrete_model
    }

    fn __str__(&self) -> String {
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.borrow())
    }

    #[pyo3(signature=(compress=true, level=3))]
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        let compress = compress.unwrap_or(level.is_some());
        Ok(PyBytes::new(
            py,
            &self
                .borrow()
                .encode()
                .maybe_compress(compress, level)?
                .versionize(),
        )
            .into())
    }

    /// Alias for serialize
    #[pyo3(signature=(compress=true, level=3))]
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
        Ok(Self::new(
            data.as_bytes(py).unversionize().decompress()?.decode(())?,
        ))
    }

    #[staticmethod]
    fn deserialize(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::decode(py, data)
    }

    fn evaluate(&self, solution: &PySolution) -> PySolution {
        PySolution(
            self.borrow()
                .evaluate_solution(RcSolution::clone(&solution.0)),
        )
    }

    fn evaluate_sample(&self, sample: &PySample) -> PyOwnedResult {
        PyOwnedResult(self.borrow().evaluate_sample(&sample.0))
    }
}

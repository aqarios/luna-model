use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use super::py_bounds::BoundValue;
use super::py_constr::PyConstraint;
use super::py_model_metadata::PyModelMetadata;
use super::py_utilities::{repr_model, Replacement};
use super::{
    py_constr::PyConstraints, py_env::PyEnvironment, py_expr::PyExpression, py_sol::PySolution,
};
use crate::core::environment::SharedEnvironment;
use crate::core::operations::AddAssignToExpression;
use crate::core::{ContentEquality, LazyBounds, SharedSolution, Sense, Vtype};
use crate::hashing::hash_model;
use crate::py_bindings::py_res::PyOwnedResult;
use crate::py_bindings::py_sample::PySample;
use crate::py_bindings::py_var::PyVariable;
use crate::{
    core::Model,
    py_bindings::py_env::CURRENT_ENV,
    serialization::{
        Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
    },
};
use derive_more::{Deref, DerefMut};
use either::Either::{Left, Right};
use pyo3::types::PyType;
use pyo3::{prelude::*, types::PyBytes};

/// A symbolic optimization model consisting of an objective and constraints.
///
/// The `Model` class represents a structured symbolic optimization problem. It
/// combines a scalar objective `Expression`, a collection of `Constraints`, and
/// a shared `Environment` that scopes all variables used in the model.
///
/// Models can be constructed explicitly by passing an environment, or implicitly
/// by allowing the model to create its own private environment. If constructed
/// inside an active `Environment` context (via `with Environment()`), that context
/// is used automatically.
///
/// Parameters
/// ----------
/// env : Environment, optional
///     The environment in which variables and expressions are created. If not
///     provided, the model will either use the current context (if active), or
///     create a new private environment.
/// name : str, optional
///     An optional name assigned to the model.
///
/// Examples
/// --------
/// Basic usage:
///
/// >>> from luna_quantum import Model, Variable
/// >>> model = Model("MyModel")
/// >>> with model.environment:
/// ...     x = Variable("x")
/// ...     y = Variable("y")
/// >>> model.objective = x * y + x
/// >>> model.constraints += x >= 0
/// >>> model.constraints += y <= 5
///
/// With explicit environment:
///
/// >>> from luna_quantum import Environment
/// >>> env = Environment()
/// >>> model = Model("ScopedModel", env)
/// >>> with env:
/// ...     x = Variable("x")
/// ...     model.objective = x * x
///
/// Serialization:
///
/// >>> blob = model.encode()
/// >>> restored = Model.decode(blob)
/// >>> restored.name
/// 'MyModel'
///
/// Notes
/// -----
/// - The `Model` class does not solve the optimization problem.
/// - Use `.objective`, `.constraints`, and `.environment` to access the symbolic content.
/// - Use `encode()` and `decode()` to serialize and recover models.
#[cfg_attr(
    not(feature = "lq"),
    pyclass(unsendable, subclass, name = "Model", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(unsendable, subclass, name = "Model", module = "luna_quantum._core")
)]
#[derive(Clone, Deref, DerefMut)]
pub struct PyModel {
    #[deref]
    #[deref_mut]
    pub concrete_model: Rc<RefCell<Model>>,
    #[pyo3(get, set)]
    pub _metadata: PyModelMetadata,
}

impl PyModel {
    pub fn new(model: Model) -> Self {
        Self {
            concrete_model: Rc::new(RefCell::new(model)),
            _metadata: PyModelMetadata::new(),
        }
    }
}

// impl Into<Rc<RefCell<Model>>> for PyModel {
//     fn into(self) -> Rc<RefCell<Model>> {
//         self.concrete_model
//     }
// }

#[pymethods]
impl PyModel {
    /// Initialize a new symbolic model.
    ///
    /// Parameters
    /// ----------
    /// name : str, optional
    ///     An optional name for the model.
    /// env : Environment, optional
    ///     The environment in which the model operates. If not provided, a new
    ///     environment will be created or inferred from context.
    #[new]
    #[pyo3(signature=(name=None, sense=None, env=None))]
    fn py_new(name: Option<String>, sense: Option<Sense>, env: Option<PyEnvironment>) -> Self {
        let env: PyEnvironment = match env {
            Some(env) => env.clone(),
            None => CURRENT_ENV.with(|curr| {
                curr.borrow()
                    .clone()
                    .unwrap_or_else(|| PyEnvironment::new(SharedEnvironment::default()))
            }),
        };
        Self::new(Model::new_with_env(name, sense, env.0))
    }

    /// Add a new variable to the model.
    ///
    /// Parameters
    /// ----------
    /// name : str
    ///     The name of the variable.
    /// vtype : Vtype, optional
    ///     The variable type (e.g., `Vtype.Real`, `Vtype.Integer`, etc.).
    ///     Defaults to `Vtype.Binary`.
    /// lower: float, optional
    ///     The lower bound restricts the range of the variable. Only applicable for
    ///     `Real` and `Integer` variables.
    /// upper: float, optional
    ///     The upper bound restricts the range of the variable. Only applicable for
    ///     `Real` and `Integer` variables.
    ///
    /// Returns
    /// -------
    /// Variable
    ///     The variable added to the model.
    #[pyo3(signature = (name, vtype=None, lower=BoundValue::None, upper=BoundValue::None))]
    fn add_variable(
        &self,
        name: String,
        vtype: Option<Vtype>,
        lower: BoundValue,
        upper: BoundValue,
    ) -> PyResult<PyVariable> {
        let bounds = match (&lower, &upper) {
            (BoundValue::None, BoundValue::None) => None,
            _ => Some(LazyBounds::new(lower.into(), upper.into())),
        };
        Ok(PyVariable::new(
            self.concrete_model
                .borrow()
                .environment
                .add_variable(&name, vtype, bounds)?,
        ))
    }

    /// Get a variable by its label (name).
    ///
    /// Parameters
    /// ----------
    /// label : str
    ///     The name/label of the variable
    ///
    /// Returns
    /// -------
    /// Variable
    ///     The variable with the specified label/name.
    ///
    /// Raises
    /// ------
    /// VariableNotExistingError
    ///     If no variable with the specified name is registered.
    fn get_variable(&self, name: String) -> PyResult<PyVariable> {
        Ok(PyVariable(Rc::new(
            self.borrow().environment.get_vref_by_name(&name)?,
        )))
    }

    /// Set the optimization sense of a model.
    ///
    /// Parameters
    /// ----------
    /// sense : Sense
    ///     The sense of the model (minimization, maximization)
    #[pyo3(name = "set_sense")]
    fn set_sense_py(&mut self, sense: Sense) {
        self.borrow_mut().set_sense(sense);
    }

    /// Get the sense of the model
    ///
    /// Returns
    /// -------
    /// Sense
    ///     The sense of the model (Min or Max).
    #[getter]
    fn get_sense(&self) -> Sense {
        self.borrow().sense
    }

    /// Get the objective expression of the model.
    #[getter]
    fn get_objective(&self) -> PyExpression {
        PyExpression::with_parent(Rc::clone(&self))
    }

    /// Set the objective expression of the model.
    #[setter]
    fn set_objective(&mut self, value: &PyExpression) {
        self.borrow_mut().objective = value.get_cloned_expression();
    }

    /// Access the set of constraints associated with the model.
    #[getter]
    fn get_constraints(&self) -> PyConstraints {
        PyConstraints::with_parent(Rc::clone(&self))
    }

    /// Replace the model's constraints with a new set.
    #[setter]
    fn set_constraints(&mut self, value: &PyConstraints) {
        self.borrow_mut().constraints = value.get_cloned_constraints();
    }

    /// Add a constraint to the model's constraint collection.
    ///
    /// Parameters
    /// ----------
    /// constraint : Constraint
    ///     The constraint to be added.
    /// name : str, optional
    ///     The name of the constraint to be added.
    #[pyo3(signature=(constraint, name=None))]
    fn add_constraint(&mut self, constraint: PyConstraint, name: Option<String>) -> PyResult<()> {
        constraint.borrow_mut().set_name(name)?;
        self.borrow_mut()
            .constraints
            .add_assign(constraint.borrow().deref())?;
        Ok(())
    }

    /// Set the model's objective to this expression.
    ///
    /// Parameters
    /// ----------
    /// expression : Expression
    ///     The expression assigned to the model's objective.
    /// sense : Sense, optional
    ///     The sense of the model for this objective, by default Sense.Min.
    #[pyo3(name = "set_objective", signature=(expression, sense=None))]
    fn set_objective_direct(&mut self, expression: PyExpression, sense: Option<Sense>) -> () {
        let sense = sense.unwrap_or(self.borrow().sense);
        self.borrow_mut().set_sense(sense);
        self.borrow_mut().objective = expression.get_cloned_expression();
    }

    fn add_objective(&mut self, expression: PyExpression) -> PyResult<()> {
        Ok(match &expression.0 {
            Left(expr) => self.borrow_mut().objective.add_assign(expr)?,
            Right(parent) => self
                .borrow_mut()
                .objective
                .add_assign(&parent.borrow().objective)?,
        })
    }

    /// Return the number of constraints defined in the model.
    ///
    /// Returns
    /// -------
    /// int
    ///     Total number of constraints.
    #[getter]
    fn num_constraints(&self) -> usize {
        self.borrow().constraints.len()
    }

    /// Return the name of the model.
    #[getter]
    fn name(&self) -> String {
        self.borrow().name.clone()
    }

    /// Get the environment in which this model is defined.
    #[getter]
    fn environment(&self) -> PyEnvironment {
        PyEnvironment(self.borrow().environment.clone())
    }

    /// Get all variables that are part of this model.
    ///
    /// Parameters
    /// ----------
    /// active : bool, optional
    ///     Instead of all variables from the environment, return only those that are
    ///     actually present in the model's objective.
    ///
    /// Returns
    /// -------
    /// The model's variables as a list.
    #[pyo3(signature=(active=None))]
    fn variables(&self, active: Option<bool>) -> Vec<PyVariable> {
        let model = self.borrow();
        let active_vars = &model.objective.active;
        self.borrow()
            .environment
            .vrefs()
            .into_iter()
            .enumerate()
            .filter(|(a, _)| {
                *active_vars.get(*a as usize).unwrap_or(&false) || !active.unwrap_or_default()
            })
            .map(|(_, vref)| {
                PyVariable::new(vref)
            })
            .collect()
    }

    /// Get all model constraints that are violated by the given sample.
    ///
    /// Parameters
    /// ----------
    /// sample : Sample
    ///     The sample to check constraint feasibility for.
    ///
    /// Returns
    /// -------
    /// Constraints
    ///     The constraints violated by the given sample.
    fn violated_constraints(&self, sample: &PySample) -> PyConstraints {
        PyConstraints {
            data: Left(self.concrete_model.borrow().violated_constraints(sample)),
        }
    }

    /// Check whether this model is equal to ``other``.
    ///
    /// Parameters
    /// ----------
    /// other : Model
    ///
    /// Returns
    /// -------
    /// bool
    fn __eq__(&self, other: &Self) -> bool {
        self.concrete_model == other.concrete_model
    }

    fn __str__(&self) -> String {
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        repr_model(self)
    }

    /// Serialize the model into a compact binary format.
    ///
    /// Parameters
    /// ----------
    /// compress : bool, optional
    ///     Whether to compress the binary output. Default is True.
    /// level : int, optional
    ///     Compression level (0–9). Default is 3.
    ///
    /// Returns
    /// -------
    /// bytes
    ///     Encoded model representation.
    ///
    /// Raises
    /// ------
    /// IOError
    ///     If serialization fails.
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

    /// Alias for `encode()`.
    ///
    /// See `encode()` for full documentation.
    #[pyo3(signature=(compress=true, level=3))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<PyObject> {
        self.encode(py, compress, level)
    }

    /// Reconstruct an expression from encoded bytes.
    ///
    /// Parameters
    /// ----------
    /// data : bytes
    ///     Binary blob returned by `encode()`.
    ///
    /// Returns
    /// -------
    /// Expression
    ///     Deserialized expression object.
    ///
    /// Raises
    /// ------
    /// DecodeError
    ///     If decoding fails due to corruption or incompatibility.
    #[classmethod]
    fn decode(_cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Ok(Self::new(
            data.as_bytes(py).unversionize().decompress()?.decode(())?,
        ))
    }

    /// Alias for `decode()`.
    ///
    /// See `decode()` for full documentation.
    #[classmethod]
    fn deserialize(cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::decode(cls, py, data)
    }

    /// Evaluate the model given a solution.
    ///
    /// Parameters
    /// ----------
    /// solution : Solution
    ///     The solution used to evaluate the model with.
    ///
    /// Returns
    /// -------
    /// Solution
    ///     A new solution object with filled-out information.
    fn evaluate(&self, solution: &PySolution) -> PyResult<PySolution> {
        Ok(PySolution(
            self.borrow()
                .evaluate_solution(SharedSolution::clone(&solution.0))?,
        ))
    }

    /// Evaluate the model given a single sample.
    ///
    /// Parameters
    /// ----------
    /// sample : Sample
    ///     The sample used to evaluate the model with.
    ///
    /// Returns
    /// -------
    /// Result
    ///     A result object containing the information from the evaluation process.
    fn evaluate_sample(&self, sample: &PySample) -> PyResult<PyOwnedResult> {
        Ok(PyOwnedResult(self.borrow().evaluate_sample(&sample.0)?))
    }

    /// Substitute every occurrence of a variable in the model’s objective and constraint expressions with another expression.
    ///
    /// Given a `Model` instance `self`, this method replaces all occurrences of `target`
    /// with `replacement` for the objective and each constraint. If any substitution would
    /// cross differing environments (e.g. captures from two different scopes), it raises
    /// a `DifferentEnvsError`.
    ///
    /// Parameters
    /// ----------
    /// target : VarRef
    ///     The variable reference to replace.
    /// replacement : Expression
    ///     The expression to insert in place of `target`.
    ///
    /// Returns
    /// -------
    /// None
    ///     Performs substitution in place; no return value.
    ///
    /// Raises
    /// ------
    /// DifferentEnvsError
    ///     If the environments of `self`, `target`, and `replacement`
    ///     are not compatible.
    fn substitute(&mut self, target: &PyVariable, replacement: Replacement) -> PyResult<()> {
        let mutmodel = &mut self.concrete_model.borrow_mut();
        Ok(match &replacement.as_expr().0 {
            Left(expr) => mutmodel.substitute(&target.0, expr)?,
            Right(model) => mutmodel.substitute(&target.0, &model.borrow().objective)?,
        })
    }

    /// Compute the hash of the variable.
    fn __hash__(&self) -> PyResult<u64> {
        self.hash()
    }

    fn equal_contents(&self, other: &Self) -> bool {
        self.concrete_model
            .borrow()
            .is_equal_contents(&other.concrete_model.borrow())
    }

    // Deep clones the model
    fn deep_clone(&self) -> PyModel {
        let model = self.concrete_model.borrow().deep_clone();
        PyModel::new(model)
    }
}

impl PyModel {
    // #[pyo3(signature=(version=false, compress=false, level=None))]

    /// Compute the hash of the variable, with more options to determine how the hash is
    /// computed.
    ///
    /// WARNING: These values will not be equal to `__hash__` results due to additional
    /// implementation details in the `__hash__` function.
    fn hash(&self) -> PyResult<u64> {
        Ok(hash_model(&self.borrow()))
    }
}

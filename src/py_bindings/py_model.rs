use super::py_bounds::BoundValue;
use super::py_constr::PyConstraint;
use super::py_model_metadata::PyModelMetadata;
use super::py_sample::PySampleInner;
use super::py_translator::{PyBqmTranslator, PyCqmTranslator, PyLpTranslator, PyQuboTranslator};
use super::py_utilities::{repr_model, Replacement};
use super::{
    py_constr::PyConstraintCollection, py_env::PyEnvironment, py_expr::PyExpression,
    py_sol::PySolution,
};
use crate::core::environment::SharedEnvironment;
use crate::core::operations::AddAssignToExpression;
use crate::core::{ContentEquality, LazyBounds, Sample, Sense, Vtype};
use crate::hashing::hash_model;
use crate::py_bindings::py_res::PyOwnedResult;
use crate::py_bindings::py_sample::PySample;
use crate::py_bindings::py_specs::PyModelSpecs;
use crate::py_bindings::py_var::PyVariable;
use crate::py_bindings::unwind;
use crate::utils::{Share, ShareMut};
use crate::{
    core::Model,
    py_bindings::py_env::CURRENT_ENV,
    serialization::{Decodable, Decompressable, Encodable, Unversionizable},
};
use derive_more::{Deref, DerefMut};
use either::Either::{Left, Right};
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::ffi::c_str;
use pyo3::types::{PyDict, PyType};
use pyo3::IntoPyObjectExt;
use pyo3::{prelude::*, types::PyBytes};
use std::ffi::CStr;
use std::ops::Deref;
use std::path::PathBuf;
use strum_macros::Display;
use unwind_macros::unwindable;

static PY_REDUCE_IMPORT: &'static CStr = c_str!("from luna_model import Model");

#[pyclass(eq, name = "TranslationTarget", module = "luna_model._core")]
#[derive(Debug, Display, Hash, PartialEq)]
pub enum TranslationTarget {
    Qubo,
    Lp,
    Bqm,
    Cqm,
}

impl TranslationTarget {
    fn retrieve(py: Python, other: &Py<PyAny>) -> PyResult<TranslationTarget> {
        let builtins = PyModule::import(py, "builtins")?;
        let the_type: Py<PyAny> = builtins.getattr("type")?.call1((other,))?.extract()?;
        let type_name: String = builtins.getattr("str")?.call1((the_type,))?.extract()?;
        if type_name.contains("dimod") && type_name.contains("BinaryQuadraticModel") {
            Ok(TranslationTarget::Bqm)
        } else if type_name.contains("dimod") && type_name.contains("ConstrainedQuadraticModel") {
            Ok(TranslationTarget::Cqm)
        } else if type_name.contains("numpy.matrix") || type_name.contains("numpy.ndarray") {
            Ok(TranslationTarget::Qubo)
        } else if type_name.contains("'str'") || type_name.contains("Path") {
            Ok(TranslationTarget::Lp)
        } else {
            Err(PyTypeError::new_err(
                "translation from a '{type_name}' is not supported",
            ))
        }
    }
}

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
/// >>> from luna_model import Model, Variable
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
/// >>> from luna_model import Environment
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
#[pyclass(subclass, name = "Model", module = "luna_model._core")]
#[derive(Clone, Deref, DerefMut)]
pub struct PyModel {
    #[deref]
    #[deref_mut]
    pub concrete_model: ShareMut<Model>,
    #[pyo3(get, set)]
    pub _metadata: PyModelMetadata,
}

impl PyModel {
    pub fn new(model: Model) -> Self {
        Self {
            concrete_model: ShareMut::new(model),
            _metadata: PyModelMetadata::new(),
        }
    }
}

#[unwindable]
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
            Option::None => CURRENT_ENV.with(|curr| {
                curr.borrow()
                    .clone()
                    .unwrap_or_else(|| PyEnvironment::new(SharedEnvironment::default()))
            }),
        };
        Self::new(Model::new_with_env(name, sense, env.0))
    }

    #[staticmethod]
    #[pyo3(signature=(other, kwargs=None))]
    fn from_(py: Python, other: Py<PyAny>, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        use TranslationTarget::*;

        let source = TranslationTarget::retrieve(py, &other)?;
        match source {
            Qubo => {
                let offset = extract_maybe(&kwargs, "offset")?;
                let variable_names = extract_maybe(&kwargs, "variable_names")?;
                let name = extract_maybe(&kwargs, "name")?;
                let vtype = extract_maybe(&kwargs, "vtype")?;
                PyQuboTranslator::to_aq(other.extract(py)?, offset, variable_names, name, vtype)
            }
            Lp => PyLpTranslator::to_aq(py, other),
            Bqm => PyBqmTranslator::to_aq(py, other, extract_maybe(&kwargs, "name")?)?.extract(py),
            Cqm => PyCqmTranslator::to_aq(py, other, extract_maybe(&kwargs, "name")?)?.extract(py),
        }
    }

    #[pyo3(signature=(target, filepath=None))]
    fn to(
        &self,
        py: Python,
        target: &TranslationTarget,
        filepath: Option<PathBuf>,
    ) -> PyResult<Py<PyAny>> {
        use TranslationTarget::*;

        if *target != Lp && filepath.is_some() {
            return Err(PyValueError::new_err(
                "filepath can only be used with target 'Lp'",
            ));
        }

        match target {
            Qubo => PyQuboTranslator::from_aq(&self).map(|pyqubo| pyqubo.into_py_any(py))?,
            Lp => PyLpTranslator::from_aq(&self, filepath).map(|lp| lp.into_py_any(py))?,
            Bqm => PyBqmTranslator::from_aq(py, &self),
            Cqm => PyCqmTranslator::from_aq(py, &self),
        }
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
                .access_mut()
                .environment
                .add_variable(&name, vtype.unwrap_or_else(|| Vtype::default()), bounds)?,
        ))
    }

    /// Add a new variable to the model with fallback renaming.
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
    fn add_variable_with_fallback(
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
                .access_mut()
                .environment
                .add_variable_with_fallback(&name, vtype.unwrap_or_else(|| Vtype::default()), bounds, None)?,
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
        Ok(PyVariable(Share::new(
            self.access().environment.get_vref_by_name(&name)?,
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
        self.access_mut().set_sense(sense);
    }

    /// Get the sense of the model
    ///
    /// Returns
    /// -------
    /// Sense
    ///     The sense of the model (Min or Max).
    #[getter]
    fn get_sense(&self) -> Sense {
        self.access().sense
    }

    /// Get the objective expression of the model.
    #[getter]
    fn get_objective(&self) -> PyExpression {
        PyExpression::with_parent(self.concrete_model.clone())
    }

    /// Set the objective expression of the model.
    #[setter]
    fn set_objective(&mut self, value: &PyExpression) {
        self.access_mut().objective = value.get_cloned_expression();
    }

    /// Access the set of constraints associated with the model.
    #[getter]
    fn get_constraints(&self) -> PyConstraintCollection {
        PyConstraintCollection::with_parent(self.concrete_model.clone())
    }

    /// Replace the model's constraints with a new set.
    #[setter]
    fn set_constraints(&mut self, value: &PyConstraintCollection) {
        self.access_mut().constraints = value.get_cloned_constraints();
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
        if let Some(n) = name {
            constraint.access_mut().set_name(n)?;
        };
        self.access()
            .constraints
            .add_assign(constraint.access().deref())?;
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
        let mut slf = self.access_mut();
        let sense = sense.unwrap_or(slf.sense);
        slf.set_sense(sense);
        slf.objective = expression.get_cloned_expression();
    }

    fn add_objective(&mut self, expression: PyExpression) -> PyResult<()> {
        Ok(match &expression.0 {
            Left(expr) => self.access_mut().objective.add_assign(expr)?,
            Right(parent) => self
                .access_mut()
                .objective
                .add_assign(&parent.access().objective)?,
        })
    }

    /// Return the number of variables defined in the model.
    ///
    /// Returns
    /// -------
    /// int
    ///     Total number of variables.
    #[getter]
    fn num_variables(&self) -> usize {
        self.access().num_variables()
    }

    /// Return the number of constraints defined in the model.
    ///
    /// Returns
    /// -------
    /// int
    ///     Total number of constraints.
    #[getter]
    fn num_constraints(&self) -> usize {
        self.access().constraints.len()
    }

    /// Return the name of the model.
    #[getter]
    fn get_name(&self) -> String {
        self.access().name.clone()
    }

    /// Return the name of the model.
    #[setter]
    fn set_name(&self, name: String) {
        self.access().name = name;
    }

    /// Get the environment in which this model is defined.
    #[getter]
    fn environment(&self) -> PyEnvironment {
        PyEnvironment(self.access().environment.clone())
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
        let model = self.access_mut();
        let active_vars = &model.objective.active;
        model
            .environment
            .vrefs()
            .into_iter()
            .enumerate()
            .filter(|(a, _)| {
                active_vars.get(*a as usize).map_or_else(|| false, |r| *r)
                    || !active.unwrap_or_default()
            })
            .map(|(_, vref)| PyVariable::new(vref))
            .collect()
    }

    /// Get a list of all unique variable types of all variables in this model.
    #[pyo3(name = "vtypes")]
    fn get_vtypes(&self) -> Vec<Vtype> {
        self.access().vtypes()
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
    fn violated_constraints(&self, sample: &PySample) -> PyConstraintCollection {
        match &sample.0 {
            PySampleInner::View(view) => {
                let binding = view.sol.access();
                let samples = binding.samples();
                let sample = samples.get_sample(view.row).unwrap();
                PyConstraintCollection {
                    data: Left(self.concrete_model.access().violated_constraints(&sample)),
                }
            }
            PySampleInner::Owned(owned) => PyConstraintCollection {
                data: Left(
                    self.concrete_model
                        .access()
                        .violated_constraints(&Sample::Owned(owned.0.clone())),
                ),
            },
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
        if self.ptr_eq(other) {
            true
        } else {
            self.concrete_model
                .access()
                .eq(&other.concrete_model.access())
        }
    }

    fn __str__(&self) -> String {
        self.access().to_string()
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
    fn encode(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, &self.access().encode(compress, level)?).into())
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
    ) -> PyResult<Py<PyAny>> {
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
    fn evaluate(&self, solution: PySolution) -> PyResult<PySolution> {
        Ok(PySolution::new(
            self.access().evaluate_solution(solution.access().clone())?,
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
        match &sample.0 {
            PySampleInner::View(view) => {
                let binding = view.sol.access();
                let samples = binding.samples();
                let s = samples.get_sample(view.row).unwrap();
                Ok(PyOwnedResult::new(self.access().evaluate_sample(&s)?))
            }
            PySampleInner::Owned(owned) => Ok(PyOwnedResult::new(
                self.access()
                    .evaluate_sample(&Sample::Owned(owned.0.clone()))?,
            )),
        }
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
        let mutmodel = &mut self.concrete_model.access_mut();
        Ok(match &replacement.as_expr().0 {
            Left(expr) => mutmodel.substitute(&target.0, expr)?,
            Right(model) => mutmodel.substitute(&target.0, &model.access().objective)?,
        })
    }

    /// Compute the hash of the variable.
    fn __hash__(&self) -> PyResult<u64> {
        self.hash()
    }

    fn __reduce__(&self, py: Python) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
        py.run(PY_REDUCE_IMPORT, None, None)?;
        let decode = py.eval(c_str!("Model.decode"), None, None)?;
        let data = self.encode(py, Some(true), Some(3))?;
        Ok::<(Py<PyAny>, Py<PyAny>), PyErr>((decode.into_py_any(py)?, (data,).into_py_any(py)?))
    }

    fn equal_contents(&self, other: &Self) -> bool {
        if self.ptr_eq(other) {
            true
        } else {
            self.concrete_model
                .access()
                .is_equal_contents(&other.concrete_model.access())
        }
    }

    // Deep clones the model
    fn deep_clone(&self) -> PyModel {
        let model = self.concrete_model.access().deep_clone();
        PyModel::new(model)
    }

    fn get_specs(&self) -> PyModelSpecs {
        PyModelSpecs(self.concrete_model.access().get_specs())
    }

    fn satisfies(&self, specs: PyModelSpecs) -> bool {
        self.concrete_model.access().satisfies(specs.0)
    }

    // LunaQuantum specifics.
    /// Get the model's metadata.
    #[getter]
    fn get_metadata(&self) -> PyResult<()> {
        Err(PyRuntimeError::new_err(
            "This functionality is only available with luna_quantum: https://docs.aqarios.com",
        ))
    }

    /// Set the model's metadata.
    #[setter]
    fn set_metadata(&self, metadata: Bound<PyAny>) -> PyResult<()> {
        _ = metadata;
        Err(PyRuntimeError::new_err(
            "This functionality is only available with luna_quantum: https://docs.aqarios.com",
        ))
    }

    #[staticmethod]
    fn load_luna(model_id: String, client: Bound<PyAny>) -> PyResult<()> {
        _ = model_id;
        _ = client;
        Err(PyRuntimeError::new_err(
            "This functionality is only available with luna_quantum: https://docs.aqarios.com",
        ))
    }

    fn save_luna(&self, client: Bound<PyAny>) -> PyResult<()> {
        _ = client;
        Err(PyRuntimeError::new_err(
            "This functionality is only available with luna_quantum: https://docs.aqarios.com",
        ))
    }

    fn delete_luna(&self, client: Bound<PyAny>) -> PyResult<()> {
        _ = client;
        Err(PyRuntimeError::new_err(
            "This functionality is only available with luna_quantum: https://docs.aqarios.com",
        ))
    }

    fn load_solutions(&self, client: Bound<PyAny>) -> PyResult<()> {
        _ = client;
        Err(PyRuntimeError::new_err(
            "This functionality is only available with luna_quantum: https://docs.aqarios.com",
        ))
    }

    fn load_solve_jobs(&self, client: Bound<PyAny>) -> PyResult<()> {
        _ = client;
        Err(PyRuntimeError::new_err(
            "This functionality is only available with luna_quantum: https://docs.aqarios.com",
        ))
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
        Ok(hash_model(&self.access()))
    }
}

fn extract_maybe<'a, T: FromPyObject<'a>>(
    kwargs: &Option<&Bound<'a, PyDict>>,
    key: &str,
) -> PyResult<Option<T>> {
    if let Some(kw) = kwargs {
        if let Some(maybe_bound_val) = kw.get_item(key)? {
            Ok(Some(maybe_bound_val.extract()?))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

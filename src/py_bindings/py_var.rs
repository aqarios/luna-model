use std::cell::RefCell;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

use super::py_constr::PyConstraint;
use super::py_env::{PyEnvironment, CURRENT_ENV};
use super::py_exceptions::NoActiveEnvironmentFoundError;
use super::{py_bounds::PyBounds, py_expr::PyExpression};
use crate::core::expression::ExpressionBaseCreation;
use crate::core::operations::{
    AddToExpression, MulAssignToExpression, MulToExpression, NegToExpression, RSubToExpression,
    SubAssignToExpression, SubToExpression,
};
use crate::core::{
    environment, Comparator, ConcreteConstraint, ConcreteExpression, ConcreteRcVarRef,
    ConcreteVarRef, Expression, Vtype,
};
use derive_more::{Deref, DerefMut};
use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;

/// Represents a symbolic variable within an optimization environment.
///
/// A `Variable` is the fundamental building block of algebraic expressions
/// used in optimization models. Each variable is tied to an `Environment`
/// which scopes its lifecycle and expression context. Variables can be
/// typed and optionally bounded.
///
/// Parameters
/// ----------
/// name : str
///     The name of the variable.
/// env : Environment, optional
///     The environment in which this variable is created. If not provided,
///     the current environment from the context manager is used.
/// vtype : Vtype, optional
///     The variable type (e.g., `Vtype.Real`, `Vtype.Integer`, etc.).
///     Defaults to `Vtype.Binary`.
/// bounds : Bounds, optional
///     Bounds restricting the range of the variable. Only applicable for
///     `Real` and `Integer` variables.
///
/// Examples
/// --------
/// >>> from luna_quantum import Variable, Environment, Vtype, Bounds
/// >>> with Environment():
/// ...     x = Variable("x")
/// ...     y = Variable("y", vtype=Vtype.Integer, bounds=Bounds(0, 5))
/// ...     expr = 2 * x + y - 1
///
/// Arithmetic Overloads
/// --------------------
/// Variables support standard arithmetic operations:
///
/// - Addition: `x + y`, `x + 2`, `2 + x`
/// - Subtraction: `x - y`, `3 - x`
/// - Multiplication: `x * y`, `2 * x`, `x * 2`
///
/// All expressions return `Expression` objects and preserve symbolic structure.
///
/// Notes
/// -----
/// - A `Variable` is bound to a specific `Environment` instance.
/// - Variables are immutable; all operations yield new `Expression` objects.
/// - Variables carry their environment, but the environment does not own the variable.
#[pyclass(unsendable, subclass, name = "Variable", module = "aqmodels")]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyVariable(pub ConcreteRcVarRef);

impl PyVariable {
    pub fn new(varref: ConcreteVarRef) -> Self {
        Self(varref.into())
    }

    pub fn name(&self) -> String {
        let idx: usize = self.id.into();
        let name = &self.env.borrow().variables[idx].name;
        name.clone()
    }

    pub fn bounds(&self) -> PyBounds {
        let idx: usize = self.id.into();
        let bounds = self.env.borrow().variables[idx].bounds;
        PyBounds(bounds.into())
    }
}

impl Hash for PyVariable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state)
    }
}

impl PartialEq<Self> for PyVariable {
    fn eq(&self, other: &Self) -> bool {
        let self_idx: usize = self.id.into();
        let other_idx: usize = other.id.into();
        self.env.borrow().variables[self_idx] == other.env.borrow().variables[other_idx]
    }
}

impl Eq for PyVariable {}

#[pymethods]
impl PyVariable {
    /// Initialize a new Variable.
    ///
    /// See class-level docstring for full usage.
    ///
    /// Raises
    /// ------
    /// NoActiveEnvironmentFoundError
    ///     If no active environment is found and none is explicitly provided.
    /// VariableExistsError
    ///     If a variable with the same name already exists in the environment.
    /// VariableCreationError
    ///     If the variable is tried to be created with incompatible bounds.
    #[new]
    #[pyo3(signature=(name, vtype=None, bounds=None, env=None))]
    fn py_new(
        name: String,
        vtype: Option<Vtype>,
        bounds: Option<PyBounds>,
        env: Option<&mut PyEnvironment>,
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

    /// Get the name of the variable.
    #[getter]
    fn get_name(&self) -> String {
        self.name()
    }

    /// Get the bounds of the variable.
    #[getter]
    fn get_bounds(&self) -> PyBounds {
        self.bounds()
    }

    /// Compute the hash of the variable.
    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.name().hash(&mut s);
        s.finish()
    }

    /// Add this variable to another value.
    ///
    /// Parameters
    /// ----------
    /// other : Variable, Expression, int, or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///     The resulting symbolic expression.
    ///
    /// Raises
    /// ------
    /// VariablesFromDifferentEnvsError
    ///     If the operands belong to different environments.
    /// TypeError
    ///     If the operand type is unsupported.
    fn __add__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.add(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.add(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = rhs.borrow().add(self.as_ref())?;
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }
        Ok(PyExpression::new(expr))
    }

    /// Right-hand addition for scalars.
    ///
    /// Parameters
    /// ----------
    /// other : int or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///     The resulting symbolic expression.
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If the operand type is unsupported.
    fn __radd__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        self.__add__(py, other)
    }

    /// Subtract a value from this variable.
    ///
    /// Parameters
    /// ----------
    /// other : Variable, int, or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///     The resulting symbolic expression.
    ///
    /// Raises
    /// ------
    /// VariablesFromDifferentEnvsError
    ///     If the operands belong to different environments.
    /// TypeError
    ///     If the operand type is unsupported.
    fn __sub__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.add(-rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.sub(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = rhs.borrow().mul(-1.0).add(self.as_ref())?;
            // rhs.borrow()
            //     .add(self.as_ref())
            //     .map(|e| PyExpression::new(e))
            //     .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }

        Ok(PyExpression::new(expr))
    }

    /// Subtract this variable from a scalar (right-hand subtraction).
    ///
    /// Parameters
    /// ----------
    /// other : int or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///     The resulting symbolic expression.
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If ``other`` is not a scalar.
    fn __rsub__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(PyExpression::new(self.rsub(rhs)))
        } else {
            Err(PyTypeError::new_err("unsupported type for operation"))
        }
    }

    /// Multiply this variable by another value.
    ///
    /// Parameters
    /// ----------
    /// other : Variable, Expression, int, or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///     The resulting symbolic expression.
    ///
    /// Raises
    /// ------
    /// VariablesFromDifferentEnvsError
    ///     If the operands belong to different environments.
    /// TypeError
    ///     If the operand type is unsupported.
    fn __mul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.mul(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.mul(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = rhs.borrow().mul(self.as_ref())?;
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }

        Ok(PyExpression::new(expr))
    }

    /// Right-hand multiplication for scalars.
    ///
    /// Parameters
    /// ----------
    /// other : int or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///     The resulting symbolic expression.
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If the operand type is unsupported.
    fn __rmul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        self.__mul__(py, other)
    }

    /// Raise the variable to the power specified by `other`.
    ///
    /// Parameters
    /// ----------
    /// other : int
    ///
    /// Returns
    /// -------
    /// Expression
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If the param ``modulo`` usually supported for ``__pow__`` is specified.
    fn __pow__(&self, other: usize, modparam: Option<usize>) -> PyResult<PyExpression> {
        if modparam.is_some() {
            return Err(PyRuntimeError::new_err(
                "the parameter 'mod' is not supported.",
            ));
        }
        let expr = match other {
            0 => Expression::empty(Rc::clone(&self.env)).add(1.0),
            1 => Expression::new_linear_single(Rc::clone(&self.env), self.id, 1.0),
            2 => Expression::new_quadratic(Rc::clone(&self.env), self.id, self.id, 1.0),
            _ => {
                let mut base = Expression::new_linear_single(Rc::clone(&self.env), self.id, 1.0);
                for _ in 1..other {
                    base.mul_assign(self.as_ref())?;
                }
                base
            }
        };
        Ok(PyExpression::new(expr))
    }

    /// Negate the variable, i.e., multiply it by `-1`.
    ///
    /// Returns
    /// -------
    /// Expression
    fn __neg__(&self) -> PyExpression {
        PyExpression::new(self.0.neg())
    }

    /// Create a constraint: expression == scalar.
    ///
    /// If `rhs` is of type `Variable` or `Expression` it is moved to the `lhs` in the
    /// constraint, resulting in the following constraint:
    ///
    ///     self - rhs == 0
    ///
    /// Parameters
    /// ----------
    /// rhs : float, int, Variable or Expression
    ///
    /// Returns
    /// -------
    /// Constraint
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If the right-hand side is not of type float, int, Variable or Expression.
    fn __eq__(&self, py: Python, rhs: PyObject) -> PyResult<PyConstraint> {
        self.make_constraint(py, rhs, Comparator::Eq)
    }

    /// Create a constraint: expression <= scalar.
    ///
    /// If `rhs` is of type `Variable` or `Expression` it is moved to the `lhs` in the
    /// constraint, resulting in the following constraint:
    ///
    ///     self - rhs <= 0
    ///
    /// Parameters
    /// ----------
    /// rhs : float, int, Variable or Expression
    ///
    /// Returns
    /// -------
    /// Constraint
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If the right-hand side is not of type float, int, Variable or Expression.
    fn __le__(&self, py: Python, rhs: PyObject) -> PyResult<PyConstraint> {
        self.make_constraint(py, rhs, Comparator::Le)
    }

    /// Create a constraint: expression >= scalar.
    ///
    /// If `rhs` is of type `Variable` or `Expression` it is moved to the `lhs` in the
    /// constraint, resulting in the following constraint:
    ///
    ///     self - rhs >= 0
    ///
    /// Parameters
    /// ----------
    /// rhs : float, int, Variable or Expression
    ///
    /// Returns
    /// -------
    /// Constraint
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If the right-hand side is not of type float, int, Variable or Expression.
    fn __ge__(&self, py: Python, rhs: PyObject) -> PyResult<PyConstraint> {
        self.make_constraint(py, rhs, Comparator::Ge)
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }
}

impl PyVariable {
    fn make_constraint(
        &self,
        py: Python,
        rhs: PyObject,
        comparator: Comparator,
    ) -> PyResult<PyConstraint> {
        let mut lhs = Expression::new_linear_single(Rc::clone(&self.env), self.id, 1.0);
        let bias: PyResult<f64> = if let Ok(bias) = rhs.extract::<f64>(py) {
            Ok(bias)
        } else if let Ok(var) = rhs.extract::<PyVariable>(py) {
            lhs.sub_assign(var.as_ref())?;
            Ok(0.0)
        } else if let Ok(expr) = rhs.extract::<PyExpression>(py) {
            lhs.sub_assign(expr.borrow().deref())?;
            Ok(0.0)
        } else {
            Err(PyTypeError::new_err("unsupported type for operation"))
        };
        Ok(PyConstraint::new(ConcreteConstraint::new(
            Rc::new(RefCell::new(lhs)),
            bias?,
            comparator,
            None,
        )?))
    }
}

#[pymethods]
impl Vtype {
    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

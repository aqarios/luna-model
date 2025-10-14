use pyo3::ffi::{c_str, PyEnum_Type};
use std::collections::HashMap;

use super::unwind;
use super::{
    py_constr::PyConstraint,
    py_env::{PyEnvironment, CURRENT_ENV},
    py_exceptions::NoActiveEnvironmentFoundError,
    py_utilities::Replacement,
    py_var::PyVariable,
};
use crate::core::expression::{ExpressionEvaluation, Separation};
use crate::core::{check_variables_sol, make_index_map};
use crate::py_bindings::py_sol::PySolution;
use crate::utils::ShareMut;
use crate::{
    core::{
        environment::SharedEnvironment,
        operations::{
            AddAssignToExpression, AddToExpression, MulAssignToExpression, MulToExpression,
            SubAssignToExpression, SubToExpression,
        },
        Comparator, ContentEquality, Expression, ExpressionBase, Substitution, VarRef,
    },
    errors::VariableNotExistingErr,
    types::{Bias, VarIndex},
};
use crate::{
    core::{expression::ExpressionBaseCreation, Model},
    serialization::{Decodable, Decompressable, Encodable, Unversionizable},
};
use either::Either::{self, Left, Right};
use numpy::{PyArray1, ToPyArray};
use pyo3::exceptions::PyValueError;
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyBytes, IntoPyObjectExt};
use pyo3::{exceptions::PyTypeError, types::PyType};
use unwind_macros::unwindable;

/// Polynomial expression supporting symbolic arithmetic, constraint creation, and encoding.
///
/// An `Expression` represents a real-valued mathematical function composed of variables,
/// scalars, and coefficients. Expressions may include constant, linear, quadratic, and
/// higher-order terms (cubic and beyond). They are used to build objective functions
/// and constraints in symbolic optimization models.
///
/// Expressions support both regular and in-place arithmetic, including addition and
/// multiplication with integers, floats, `Variable` instances, and other `Expression`s.
///
/// Parameters
/// ----------
/// env : Environment, optional
///     Environment used to scope the expression when explicitly instantiating it.
///     Typically, expressions are constructed implicitly via arithmetic on variables.
///
/// Examples
/// --------
/// Constructing expressions from variables:
///
/// >>> from luna_quantum import Environment, Variable
/// >>> with Environment():
/// ...     x = Variable("x")
/// ...     y = Variable("y")
/// ...     expr = 1 + 2 * x + 3 * x * y + x * y * y
///
/// Inspecting terms:
///
/// >>> expr.get_offset()
/// 1.0
/// >>> expr.get_linear(x)
/// 2.0
/// >>> expr.get_quadratic(x, y)
/// 3.0
/// >>> expr.get_higher_order((x, y, y))
/// 1.0
///
/// In-place arithmetic:
///
/// >>> expr += x
/// >>> expr *= 2
///
/// Creating constraints:
///
/// >>> constraint = expr == 10.0
/// >>> constraint2 = expr <= 15
///
/// Serialization:
///
/// >>> blob = expr.encode()
/// >>> restored = Expression.decode(blob)
///
/// Supported Arithmetic
/// --------------------
/// The following operations are supported:
///
/// - Addition:
///     * `expr + expr` → `Expression`
///     * `expr + variable` → `Expression`
///     * `expr + int | float` → `Expression`
///     * `int | float + expr` → `Expression`
///
/// - In-place addition:
///     * `expr += expr`
///     * `expr += variable`
///     * `expr += int | float`
///
/// - Multiplication:
///     * `expr * expr`
///     * `expr * variable`
///     * `expr * int | float`
///     * `int | float * expr`
///
/// - In-place multiplication:
///     * `expr *= expr`
///     * `expr *= variable`
///     * `expr *= int | float`
///
/// - Constraint creation:
///     * `expr == constant` → `Constraint`
///     * `expr <= constant` → `Constraint`
///     * `expr >= constant` → `Constraint`
///
/// Notes
/// -----
/// - Expressions are mutable: in-place operations (`+=`, `*=`) modify the instance.
/// - Expressions are scoped to an environment via the variables they reference.
/// - Comparisons like `expr == expr` return `bool`, not constraints.
/// - Use `==`, `<=`, `>=` with numeric constants to create constraints.
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "Expression", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "Expression", module = "luna_quantum._core")
)]
#[derive(Clone)]
pub struct PyExpression(pub Either<Expression, ShareMut<Model>>);

impl PyExpression {
    pub fn new(expr: Expression) -> Self {
        Self(Left(expr))
    }
    pub fn with_parent(parent: ShareMut<Model>) -> Self {
        Self(Right(parent))
    }

    pub fn get_cloned_expression(&self) -> Expression {
        match &self.0 {
            Left(expr) => expr.clone(),
            Right(parent) => parent.access().objective.clone(),
        }
    }
}

/// Iterate over the single components of an expression.
///
/// Examples
/// --------
/// >>> from luna_quantum import Constant, Expression, HigherOrder, Linear, Quadratic
/// >>> expr: Expression = ...
/// >>> vars: Constant | Linear | Quadratic | HigherOrder
/// >>> bias: float
/// >>> for vars, bias in expr.items():
/// >>> match vars:
/// >>>     case Constant(): do_something_with_constant(bias)
/// >>>     case Linear(x): do_something_with_linear_var(x, bias)
/// >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
/// >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "ExpressionIterator", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "ExpressionIterator", module = "luna_quantum._core")
)]
pub struct PyExpressionIterator {
    items: Vec<(Vec<VarIndex>, Bias)>,
    env: SharedEnvironment,
    current_idx: usize,
}

/// Convenience class to indicate the empty set of variables of an expression's
/// constant term when iterating over the expression's components.
///
/// Note that the bias corresponding to the constant part is not part of this class.
///
/// Examples
/// --------
/// >>> from luna_quantum import Constant, Expression, HigherOrder, Linear, Quadratic
/// >>> expr: Expression = ...
/// >>> vars: Constant | Linear | Quadratic | HigherOrder
/// >>> bias: float
/// >>> for vars, bias in expr.items():
/// >>> match vars:
/// >>>     case Constant(): do_something_with_constant(bias)
/// >>>     case Linear(x): do_something_with_linear_var(x, bias)
/// >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
/// >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "Constant", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "Constant", module = "luna_quantum._core")
)]
pub struct PyConstant();

/// Convenience class to indicate the variable of an expression's linear term when
/// iterating over the expression's components.
///
/// Note that the bias corresponding to this variable is not part of this class.
///
/// Examples
/// --------
/// >>> from luna_quantum import Constant, Expression, HigherOrder, Linear, Quadratic
/// >>> expr: Expression = ...
/// >>> vars: Constant | Linear | Quadratic | HigherOrder
/// >>> bias: float
/// >>> for vars, bias in expr.items():
/// >>> match vars:
/// >>>     case Constant(): do_something_with_constant(bias)
/// >>>     case Linear(x): do_something_with_linear_var(x, bias)
/// >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
/// >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "Linear", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "Linear", module = "luna_quantum._core")
)]
pub struct PyLinear(pub PyVariable);

/// Convenience class to indicate the variables of an expression's quadratic term when
/// iterating over the expression's components.
///
/// Note that the bias corresponding to these two variables is not part of this class.
///
/// Examples
/// --------
/// >>> from luna_quantum import Constant, Expression, HigherOrder, Linear, Quadratic
/// >>> expr: Expression = ...
/// >>> vars: Constant | Linear | Quadratic | HigherOrder
/// >>> bias: float
/// >>> for vars, bias in expr.items():
/// >>> match vars:
/// >>>     case Constant(): do_something_with_constant(bias)
/// >>>     case Linear(x): do_something_with_linear_var(x, bias)
/// >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
/// >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "Quadratic", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "Quadratic", module = "luna_quantum._core")
)]
pub struct PyQuadratic(pub (PyVariable, PyVariable));

/// Convenience class to indicate the set of variables of an expression's higher-order
/// term when iterating over the expression's components.
///
/// Note that the bias corresponding to these variables is not part of this class.
///
/// Examples
/// --------
/// >>> from luna_quantum import Constant, Expression, HigherOrder, Linear, Quadratic
/// >>> expr: Expression = ...
/// >>> vars: Constant | Linear | Quadratic | HigherOrder
/// >>> bias: float
/// >>> for vars, bias in expr.items():
/// >>> match vars:
/// >>>     case Constant(): do_something_with_constant(bias)
/// >>>     case Linear(x): do_something_with_linear_var(x, bias)
/// >>>     case Quadratic(x, y): do_something_with_quadratic_vars(x, y, bias)
/// >>>     case HigherOrder(ho): do_something_with_higher_order_vars(ho, bias)
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "HigherOrder", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "HigherOrder", module = "luna_quantum._core")
)]
pub struct PyHigherOrder(pub Vec<PyVariable>);

impl PyExpressionIterator {
    fn new(expr: &PyExpression) -> Self {
        Self {
            items: match &expr.0 {
                Left(expr) => expr.items(),
                Right(p) => p.access().objective.items(),
            },
            env: match &expr.0 {
                Left(expr) => expr.env.clone(),
                Right(p) => p.access().environment.clone(),
            },
            current_idx: 0,
        }
    }
}

#[unwindable]
#[pymethods]
impl PyExpression {
    /// Create a new empty expression scoped to an environment.
    ///
    /// Parameters
    /// ----------
    /// env : Environment
    ///     The environment to which this expression is bound.
    ///
    /// aises
    /// ------
    /// NoActiveEnvironmentFoundError
    ///     If no environment is provided and none is active in the context.
    #[new]
    #[pyo3(signature=(env=None))]
    pub fn py_new(env: Option<&mut PyEnvironment>) -> PyResult<Self> {
        let env: PyEnvironment = match env {
            Some(env) => env.clone(),
            None => CURRENT_ENV.with(|current| {
                current.borrow().clone().ok_or_else(|| {
                    NoActiveEnvironmentFoundError::new_err("no active environment found.")
                })
            })?,
        };
        Ok(PyExpression::new(Expression::empty(env.0)))
    }

    #[staticmethod]
    #[pyo3(name="const", signature=(val, env=None))]
    pub fn constant(val: f64, env: Option<&mut PyEnvironment>) -> PyResult<Self> {
        let env: PyEnvironment = match env {
            Some(env) => env.clone(),
            None => CURRENT_ENV.with(|current| {
                current.borrow().clone().ok_or_else(|| {
                    NoActiveEnvironmentFoundError::new_err("no active environment found.")
                })
            })?,
        };
        Ok(PyExpression::new(Expression::simple(env.0, val)))
    }

    /// Get the degree of the expression.
    fn degree(&self) -> usize {
        match &self.0 {
            Left(expr) => expr.degree(),
            Right(parent) => parent.access().objective.degree(),
        }
    }

    /// Get the constant (offset) term in the expression.
    ///
    /// Returns
    /// -------
    /// float
    ///     The constant term.
    fn get_offset(&self) -> f64 {
        match &self.0 {
            Left(expr) => expr.offset(),
            Right(parent) => parent.access().objective.offset(),
        }
    }

    /// Get the coefficient of a linear term for a given variable.
    ///
    /// Parameters
    /// ----------
    /// variable : Variable
    ///     The variable whose linear coefficient is being queried.
    ///
    /// Returns
    /// -------
    /// float
    ///     The coefficient, or 0.0 if the variable is not present.
    ///
    /// Raises
    /// ------
    /// VariableOutOfRangeError
    ///     If the variable index is not valid in this expression's environment.
    fn get_linear(&self, variable: &PyVariable) -> PyResult<f64> {
        Ok(match &self.0 {
            Left(expr) => expr.linear(variable.id)?,
            Right(parent) => parent.access().objective.linear(variable.id)?,
        })
    }

    /// Get the coefficient for a quadratic term (u * v).
    ///
    /// Parameters
    /// ----------
    /// u : Variable
    /// v : Variable
    ///
    /// Returns
    /// -------
    /// float
    ///     The coefficient, or 0.0 if not present.
    ///
    /// Raises
    /// ------
    /// VariableOutOfRangeError
    ///     If either variable is out of bounds for the expression's environment.
    fn get_quadratic(&self, u: &PyVariable, v: &PyVariable) -> PyResult<f64> {
        Ok(match &self.0 {
            Left(expr) => expr.quadratic(u.id, v.id)?,
            Right(parent) => parent.access().objective.quadratic(u.id, v.id)?,
        })
    }

    /// Get the coefficient for a higher-order term (degree ≥ 3).
    ///
    /// Parameters
    /// ----------
    /// variables : tuple of Variable
    ///     A tuple of variables specifying the term.
    ///
    /// Returns
    /// -------
    /// float
    ///     The coefficient, or 0.0 if not present.
    ///
    /// Raises
    /// ------
    /// VariableOutOfRangeError
    ///     If any variable is out of bounds for the environment.
    fn get_higher_order(&self, variables: Vec<PyVariable>) -> PyResult<f64> {
        // todo: optimize the iter away...
        Ok(match &self.0 {
            Left(expr) => expr.higher_order(&variables.iter().map(|v| v.id).collect())?,
            Right(parent) => parent
                .access()
                .objective
                .higher_order(&variables.iter().map(|v| v.id).collect())?,
        })
    }

    /// Return the number of distinct variables in the expression.
    ///
    /// Returns
    /// -------
    /// int
    ///     Number of variables with non-zero coefficients.
    #[getter]
    fn get_num_variables(&self) -> usize {
        match &self.0 {
            Left(expr) => expr.num_variables(),
            Right(parent) => parent.access().objective.num_variables(),
        }
    }

    /// Serialize the expression into a compact binary format.
    ///
    /// Parameters
    /// ----------
    /// compress : bool, optional
    ///     Whether to compress the data. Default is True.
    /// level : int, optional
    ///     Compression level (0–9). Default is 3.
    ///
    /// Returns
    /// -------
    /// bytes
    ///     Encoded representation of the expression.
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
        let base = match &self.0 {
            Left(expr) => expr,
            Right(parent) => &parent.access().objective,
        };
        Ok(PyBytes::new(py, &base.encode(compress, level)?).into())
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
    fn decode(
        _cls: &Bound<'_, PyType>,
        py: Python,
        data: Py<PyBytes>,
        env: PyEnvironment,
    ) -> PyResult<Self> {
        Ok(PyExpression::new(
            data.as_bytes(py)
                .unversionize()
                .decompress()?
                .decode(env.0)?,
        ))
    }

    /// Alias for `decode()`.
    ///
    /// See `decode()` for full documentation.
    #[classmethod]
    fn deserialize(
        cls: &Bound<'_, PyType>,
        py: Python,
        data: Py<PyBytes>,
        env: PyEnvironment,
    ) -> PyResult<Self> {
        Self::decode(cls, py, data, env)
    }

    /// Add another expression, variable, or scalar.
    ///
    /// Parameters
    /// ----------
    /// other : Expression, Variable, int, or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///
    /// Raises
    /// ------
    /// VariablesFromDifferentEnvsError
    ///     If operands are from different environments.
    /// TypeError
    ///     If the operand type is unsupported.
    fn __add__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyExpression> {
        let expr: Expression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = match &self.0 {
                Left(e) => e.add(rhs),
                Right(p) => p.access().objective.add(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = match &self.0 {
                Left(e) => e.add(rhs.as_ref())?,
                Right(p) => p.access().objective.add(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = match (&self.0, rhs.0) {
                (Left(l), Left(r)) => l.add(&r)?,
                (Left(l), Right(r)) => l.add(&r.access().objective)?,
                (Right(l), Left(r)) => l.access().objective.add(&r)?,
                (Right(l), Right(r)) => {
                    if l.ptr_eq(&r) {
                        let m = l.access();
                        m.objective.add(&m.objective)?
                    } else {
                        l.access().objective.add(&r.access().objective)?
                    }
                }
            }
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }

        Ok(PyExpression::new(expr))
    }

    /// Add this expression to a scalar or variable.
    ///
    /// Parameters
    /// ----------
    /// other : int, float, or Variable
    ///
    /// Returns
    /// -------
    /// Expression
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If the operand type is unsupported.
    fn __radd__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyExpression> {
        self.__add__(py, other)
    }

    /// Subtract another expression, variable, or scalar.
    ///
    /// Parameters
    /// ----------
    /// other : Expression, Variable, int, or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///
    /// Raises
    /// ------
    /// VariablesFromDifferentEnvsError
    ///     If operands are from different environments.
    /// TypeError
    ///     If the operand type is unsupported.
    fn __sub__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyExpression> {
        let expr: Expression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = match &self.0 {
                Left(e) => e.sub(rhs),
                Right(p) => p.access().objective.sub(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = match &self.0 {
                Left(e) => e.sub(rhs.as_ref())?,
                Right(p) => p.access().objective.sub(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = match (&self.0, rhs.0) {
                (Left(l), Left(r)) => l.sub(&r)?,
                (Left(l), Right(r)) => l.sub(&r.access().objective)?,
                (Right(l), Left(r)) => l.access().objective.sub(&r)?,
                (Right(l), Right(r)) => {
                    if l.ptr_eq(&r) {
                        let m = l.access();
                        m.objective.sub(&m.objective)?
                    } else {
                        l.access().objective.sub(&r.access().objective)?
                    }
                }
            }
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }

        Ok(PyExpression::new(expr))
    }

    /// Multiply this expression by another value.
    ///
    /// Parameters
    /// ----------
    /// other : Expression, Variable, int, or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///
    /// Raises
    /// ------
    /// VariablesFromDifferentEnvsError
    ///     If operands are from different environments.
    /// TypeError
    ///     If the operand type is unsupported.
    fn __mul__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyExpression> {
        let expr: Expression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = match &self.0 {
                Left(e) => e.mul(rhs),
                Right(p) => p.access().objective.mul(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = match &self.0 {
                Left(e) => e.mul(rhs.as_ref())?,
                Right(p) => p.access().objective.mul(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = match (&self.0, rhs.0) {
                (Left(l), Left(r)) => l.mul(&r)?,
                (Left(l), Right(r)) => l.mul(&r.access().objective)?,
                (Right(l), Left(r)) => l.access().objective.mul(&r)?,
                (Right(l), Right(r)) => {
                    if l.ptr_eq(&r) {
                        let m = l.access();
                        m.objective.mul(&m.objective)?
                    } else {
                        l.access().objective.mul(&r.access().objective)?
                    }
                }
            }
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }
        Ok(PyExpression::new(expr))
    }

    /// Right-hand multiplication.
    ///
    /// Parameters
    /// ----------
    /// other : int or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If the operand type is unsupported.
    fn __rmul__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyExpression> {
        self.__mul__(py, other)
    }

    /// Right-hand subtraction from another expression, variable, or scalar.
    ///
    /// Parameters
    /// ----------
    /// other : int, or float (lhs)
    ///
    /// Returns
    /// -------
    /// Expression
    ///
    /// Raises
    /// ------
    /// VariablesFromDifferentEnvsError
    ///     If operands are from different environments.
    /// TypeError
    ///     If the operand type is unsupported.
    fn __rsub__(&self, other: f64) -> PyResult<PyExpression> {
        let expr = match &self.0 {
            Left(e) => (-e).sub(other),
            Right(p) => (-&(p.access().objective)).sub(other),
        };

        Ok(PyExpression::new(expr))
    }

    /// In-place addition.
    ///
    /// Parameters
    /// ----------
    /// other : Expression, Variable, int, or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///
    /// Raises
    /// ------
    /// VariablesFromDifferentEnvsError
    ///     If operands are from different environments.
    /// TypeError
    ///     If the operand type is unsupported.
    pub fn __iadd__(&mut self, py: Python, other: Py<PyAny>) -> PyResult<()> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            match &mut self.0 {
                Left(e) => e.add_assign(rhs),
                Right(p) => p.access().objective.add_assign(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            match &mut self.0 {
                Left(e) => e.add_assign(rhs.as_ref())?,
                Right(p) => p.access().objective.add_assign(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            match (&mut self.0, rhs.0) {
                (Left(l), Left(r)) => l.add_assign(&r)?,
                (Left(l), Right(r)) => l.add_assign(&r.access().objective)?,
                (Right(l), Left(r)) => l.access().objective.add_assign(&r)?,
                (Right(l), Right(r)) => {
                    if l.ptr_eq(&r) {
                        let mut m = l.access();
                        let to = m.objective.clone();
                        m.objective.add_assign(&to)?
                    } else {
                        l.access_mut().objective.add_assign(&r.access().objective)?
                    }
                }
            }
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }

        Ok(())
    }

    /// In-place subtraction.
    ///
    /// Parameters
    /// ----------
    /// other : Expression, Variable, int, or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///
    /// Raises
    /// ------
    /// VariablesFromDifferentEnvsError
    ///     If operands are from different environments.
    /// TypeError
    ///     If the operand type is unsupported.
    fn __isub__(&mut self, py: Python, other: Py<PyAny>) -> PyResult<()> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            match &mut self.0 {
                Left(e) => e.sub_assign(rhs),
                Right(p) => p.access().objective.sub_assign(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            match &mut self.0 {
                Left(e) => e.sub_assign(rhs.as_ref())?,
                Right(p) => p.access().objective.sub_assign(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            match (&mut self.0, rhs.0) {
                (Left(l), Left(r)) => l.sub_assign(&r)?,
                (Left(l), Right(r)) => l.sub_assign(&r.access().objective)?,
                (Right(l), Left(r)) => l.access().objective.sub_assign(&r)?,
                (Right(l), Right(r)) => {
                    if l.ptr_eq(&r) {
                        let mut m = l.access();
                        let to = m.objective.clone();
                        m.objective.sub_assign(&to)?
                    } else {
                        l.access_mut().objective.sub_assign(&r.access().objective)?
                    }
                }
            }
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }

        Ok(())
    }

    /// In-place multiplication.
    ///
    /// Parameters
    /// ----------
    /// other : Expression, Variable, int, or float
    ///
    /// Returns
    /// -------
    /// Expression
    ///
    /// Raises
    /// ------
    /// VariablesFromDifferentEnvsError
    ///     If operands are from different environments.
    /// TypeError
    ///     If the operand type is unsupported.
    fn __imul__(&mut self, py: Python, other: Py<PyAny>) -> PyResult<()> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            match &mut self.0 {
                Left(e) => e.mul_assign(rhs),
                Right(p) => p.access().objective.mul_assign(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            match &mut self.0 {
                Left(e) => e.mul_assign(rhs.as_ref())?,
                Right(p) => p.access().objective.mul_assign(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            match (&mut self.0, rhs.0) {
                (Left(l), Left(r)) => l.mul_assign(&r)?,
                (Left(l), Right(r)) => l.mul_assign(&r.access().objective)?,
                (Right(l), Left(r)) => l.access().objective.mul_assign(&r)?,
                (Right(l), Right(r)) => {
                    if l.ptr_eq(&r) {
                        let mut m = l.access_mut();
                        let to = m.objective.clone();
                        m.objective.mul_assign(&to)?
                    } else {
                        l.access_mut().objective.mul_assign(&r.access().objective)?
                    }
                }
            }
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }
        Ok(())
    }

    /// Raise the expression to the power specified by `other`.
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
    fn __pow__(&self, other: isize, modparam: Option<isize>) -> PyResult<PyExpression> {
        // Using PyUsize as param type in a slot would still lead to a TypeError upon negative values.
        if modparam.is_some() {
            return Err(PyRuntimeError::new_err(
                "the parameter 'mod' is not supported.",
            ));
        }
        let expr = match other {
            i if i < 0 => Err(PyValueError::new_err(format!(
                "Expected a non-negative number, received: {i}"
            )))?,
            0 => {
                let env = match &self.0 {
                    Left(expr) => expr.env.clone(),
                    Right(p) => p.access().environment.clone(),
                };
                Expression::empty(env).add(1.0)
            }
            1 => match &self.0 {
                Left(expr) => expr.clone(),
                Right(p) => p.access().objective.clone(),
            },
            _ => match &self.0 {
                Left(expr) => {
                    let mut base = Expression::empty(expr.env.clone()).add(1.0);
                    for _ in 0..other {
                        base.mul_assign(expr)?;
                    }
                    base
                }
                Right(p) => {
                    let m = p.access();
                    let mut base = Expression::empty(m.environment.clone()).add(1.0);
                    for _ in 0..other {
                        base.mul_assign(&m.objective)?;
                    }
                    base
                }
            },
        };
        Ok(PyExpression::new(expr))
    }

    /// Compare two expressions for equality.
    ///
    /// Parameters
    /// ----------
    /// other : Expression
    ///     The expression to which `self` is compared to.
    ///
    /// Returns
    /// -------
    /// bool
    ///     If the two expressions are equal.
    pub fn is_equal(&self, other: &PyExpression) -> bool {
        match (&self.0, &other.0) {
            (Left(l), Left(r)) => l == r,
            (Left(l), Right(r)) => *l == r.access().objective,
            (Right(l), Left(r)) => l.access().objective == *r,
            (Right(l), Right(r)) => {
                if l.ptr_eq(&r) {
                    true
                } else {
                    l.access().objective == r.access().objective
                }
            }
        }
    }

    /// Compare to a different expression or create a constraint ``expression == scalar``
    ///
    /// If `rhs` is of type `Variable` or `Expression` it is moved to the `lhs` in the
    /// constraint, resulting in the following constraint:
    ///
    ///     self - rhs == 0
    ///
    /// Parameters
    /// ----------
    /// rhs : Expression or float, int, Variable or Expression
    ///
    /// Returns
    /// -------
    /// bool or Constraint
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If the right-hand side is not an Expression or scalar.
    fn __eq__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyConstraint> {
        PyConstraint::new_py(py, &self, other, Comparator::Eq)
    }

    /// Create a constraint ``expression <= scalar``.
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
    fn __le__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyConstraint> {
        PyConstraint::new_py(py, &self, other, Comparator::Le)
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
    fn __ge__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyConstraint> {
        PyConstraint::new_py(py, &self, other, Comparator::Ge)
    }

    /// Negate the expression, i.e., multiply it by `-1`.
    ///
    /// Returns
    /// -------
    /// Expression
    fn __neg__(&self) -> PyExpression {
        PyExpression::new(match &self.0 {
            Left(expr) => -expr,
            Right(p) => -(&p.access().objective),
        })
    }

    // /// Check whether this expression is different from ``other``.
    // ///
    // /// Parameters
    // /// ----------
    // /// other : Expression
    // ///
    // /// Returns
    // /// -------
    // /// bool
    // fn __ne__(&self, other: &Self) -> bool {
    //     *self.access() != *other.access()
    // }

    fn __str__(&self) -> String {
        match &self.0 {
            Left(expr) => expr.to_string(),
            Right(p) => p.access().objective.to_string(),
        }
    }

    fn __repr__(&self) -> String {
        match &self.0 {
            Left(expr) => format!("{:#?}", expr),
            Right(p) => format!("{:#?}", p.access().objective),
        }
    }

    /// Get this expression's environment.
    #[getter]
    fn environment(&self) -> PyEnvironment {
        PyEnvironment(match &self.0 {
            Left(expr) => expr.env.clone(),
            Right(p) => p.access().environment.clone(),
        })
    }

    /// Get all variables that are part of this expression.
    //
    /// Returns
    /// -------
    /// list[Variable]
    ///     The list of active variables
    fn variables(&self) -> Vec<PyVariable> {
        let active_vars = match &self.0 {
            Left(expr) => expr.variables(),
            Right(p) => p.access().objective.variables(),
        };
        let env = match &self.0 {
            Left(expr) => expr.env.clone(),
            Right(p) => p.access().environment.clone(),
        };

        active_vars
            .into_iter()
            .map(|id| PyVariable::new(VarRef::new(id, env.clone())))
            .collect()
    }

    /// Iterate over the single components of an expression. An *component* refers to
    /// a single constant, linear, quadratic, or higher-order term of an expression.
    //
    /// Returns
    /// -------
    /// ExpressionIterator
    ///     The iterator over the expression's components.
    fn items(&self) -> PyExpressionIterator {
        PyExpressionIterator::new(&self)
    }

    /// Get all linear components.
    //
    /// Returns
    /// -------
    /// list[tuple[Variable, float]]
    ///     The linear components.
    fn linear_items(&self) -> Vec<(PyVariable, Bias)> {
        let linear_items = match &self.0 {
            Left(expr) => expr.linear_items(),
            Right(p) => p.access().objective.linear_items(),
        };
        let env = match &self.0 {
            Left(expr) => expr.env.clone(),
            Right(p) => p.access().environment.clone(),
        };

        linear_items
            .into_iter()
            .map(|(id, bias)| (PyVariable::new(VarRef::new(id, env.clone())), bias))
            .collect()
    }

    /// Get all quadratic components.
    //
    /// Returns
    /// -------
    /// list[tuple[Variable, Variable, float]]
    ///     The quadratic components.
    fn quadratic_items(&self) -> Vec<(PyVariable, PyVariable, Bias)> {
        let quadratic_items = match &self.0 {
            Left(expr) => expr.quadratic_items(),
            Right(p) => p.access().objective.quadratic_items(),
        };
        let env = match &self.0 {
            Left(expr) => expr.env.clone(),
            Right(p) => p.access().environment.clone(),
        };

        quadratic_items
            .into_iter()
            .map(|(id1, id2, bias)| {
                (
                    PyVariable::new(VarRef::new(id1, env.clone())),
                    PyVariable::new(VarRef::new(id2, env.clone())),
                    bias,
                )
            })
            .collect()
    }

    /// Get all higher-order components.
    //
    /// Returns
    /// -------
    /// list[tuple[list[Variable], float]]
    ///     The higher-order components.
    fn higher_order_items(&self) -> Vec<(Vec<PyVariable>, Bias)> {
        let higher_order_items = match &self.0 {
            Left(expr) => expr.higher_order_items(),
            Right(p) => p.access().objective.higher_order_items(),
        };
        let env = match &self.0 {
            Left(expr) => expr.env.clone(),
            Right(p) => p.access().environment.clone(),
        };

        higher_order_items
            .into_iter()
            .map(|(ids, bias)| {
                (
                    ids.into_iter()
                        .map(|id| PyVariable::new(VarRef::new(id, env.clone())))
                        .collect(),
                    bias,
                )
            })
            .collect()
    }

    /// Substitute every occurrence of a variable in an expression with another expression.
    ///
    /// Given an expression `self`, this method replaces all occurrences of `target`
    /// with `replacement`. If the substitution would cross differing environments
    /// (e.g. captures from two different scopes), it returns a `DifferentEnvsError`.
    ///
    /// Parameters
    /// ----------
    /// target : VarRef
    ///     The variable reference to replace.
    /// replacement : Expression | Variable
    ///     The expression to insert in place of `target`.
    ///
    /// Returns
    /// -------
    /// Expression
    ///     The resulting expression after substitution.
    ///
    /// Raises
    /// ------
    /// DifferentEnvsError
    ///     If the environments of `self`, `target` and `replacement`
    ///     are not compatible.
    fn substitute(&self, target: &PyVariable, replacement: Replacement) -> PyResult<PyExpression> {
        let expr = match &self.0 {
            Left(expr) => match &replacement.as_expr().0 {
                Left(repl) => expr.substitute(&target.0, repl),
                Right(other_model) => expr.substitute(&target.0, &other_model.access().objective),
            },
            Right(model) => match &replacement.as_expr().0 {
                Left(repl) => (&model.access().objective).substitute(&target.0, &repl),
                Right(other_model) => (&model.access().objective)
                    .substitute(&target.0, &other_model.access().objective),
            },
        }?;
        Ok(PyExpression::new(expr))
    }

    fn equal_contents(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Left(lhs), Left(rhs)) => lhs.is_equal_contents(&rhs),
            (Left(lhs), Right(rhs)) => lhs.is_equal_contents(&rhs.access().objective),
            (Right(lhs), Left(rhs)) => lhs.access().objective.is_equal_contents(&rhs),
            (Right(lhs), Right(rhs)) => {
                if lhs.ptr_eq(&rhs) {
                    true
                } else {
                    lhs.access()
                        .objective
                        .is_equal_contents(&rhs.access().objective)
                }
            }
        }
    }

    fn has_quadratic(&self) -> bool {
        match &self.0 {
            Left(expr) => expr.has_quadratic(),
            Right(model) => model.access().objective.has_quadratic(),
        }
    }

    fn has_higher_order(&self) -> bool {
        match &self.0 {
            Left(expr) => expr.has_higher_order(),
            Right(model) => model.access().objective.has_higher_order(),
        }
    }

    fn is_constant(&self) -> bool {
        match &self.0 {
            Left(expr) => expr.is_constant(),
            Right(model) => model.access().objective.is_constant(),
        }
    }

    /// Separates expression into two expressions based on presence of variables.

    /// Parameters
    /// ----------
    /// variables : list[Variable]
    ///     The variables of which one must at least be present in a left term.

    /// Returns
    /// -------
    /// tuple[Expression, Expression]
    ///     Two expressions, left contains one of the variables right does not, i.e.
    ///     (contains, does not contain)
    fn separate(&self, variables: Vec<PyVariable>) -> PyResult<(PyExpression, PyExpression)> {
        let vars: Vec<VarRef> = variables.iter().map(|x| (**x.0).clone()).collect();
        let (left, right) = match &self.0 {
            Left(expr) => expr.separate(&vars),
            Right(model) => model.access().objective.separate(&vars),
        }?;
        Ok((PyExpression::new(left), PyExpression::new(right)))
    }

    #[staticmethod]
    fn deep_clone_many(py_exprs: Vec<PyExpression>) -> PyResult<Vec<PyExpression>> {
        let parent_exprs: HashMap<usize, Expression> = py_exprs
            .iter()
            .enumerate()
            .filter_map(|(i, e)| match &(e.0) {
                Left(_) => None,
                Right(parent) => Some((i, parent.access().objective.clone())),
            })
            .collect();
        let exprs: Vec<&Expression> = py_exprs
            .iter()
            .enumerate()
            .map(|(i, e)| match &(e.0) {
                Left(expr) => expr,
                Right(_) => parent_exprs.get(&i).unwrap(),
            })
            .collect();

        let cloned_exprs = Expression::deep_clone_many(&exprs)?;
        Ok(cloned_exprs
            .into_iter()
            .map(|e| PyExpression::new(e))
            .collect())
    }

    /// Evaluate model based on existing solution
    fn evaluate<'a>(&self, py: Python<'a>, sol: &PySolution) -> PyResult<Bound<'a, PyArray1<f64>>> {
        let env = match &self.0 {
            Either::Left(e) => e.env.clone(),
            Either::Right(e) => e.access().environment.clone(),
        };

        {
            let vars_sol = &sol.access().variable_names;
            let vars_env = env.variable_names();
            check_variables_sol(vars_sol, &vars_env)?;
        }

        let expr: &Expression = match &self.0 {
            Either::Left(e) => e,
            Either::Right(e) => &e.access().objective,
        };
        // Can fail if env in
        let index_map = make_index_map(sol.access().varname_to_pos(), &env);
        let res = sol
            .access()
            .iter_samples()
            .map(|x| expr.evaluate_sample(&x, |i| index_map[&i].into()))
            .collect::<Vec<f64>>()
            .to_pyarray(py);
        Ok(res)
    }

    fn __reduce__(&self, py: Python) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
        py.run(c_str!("from aqmodels import Expression"), None, None)?;
        let decode = py.eval(c_str!("Expression._decode"), None, None)?;
        let data = self.encode(py, Some(true), Some(3))?;
        let env_data = self.environment()?.encode(py, Some(true), Some(3))?;
        Ok::<(Py<PyAny>, Py<PyAny>), PyErr>((
            decode.into_py_any(py)?,
            (data, env_data).into_py_any(py)?,
        ))
    }

    #[classmethod]
    fn _decode(
        _cls: &Bound<'_, PyType>,
        py: Python,
        data: Py<PyBytes>,
        env_data: Py<PyBytes>,
    ) -> PyResult<Self> {
        let env = SharedEnvironment::from(
            env_data
                .as_bytes(py)
                .unversionize()
                .decompress()?
                .decode(())?,
        );
        Ok(PyExpression::new(
            data.as_bytes(py).unversionize().decompress()?.decode(env)?,
        ))
    }
}

#[unwindable]
#[pymethods]
impl PyExpressionIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>, py: Python) -> PyResult<Option<(Py<PyAny>, Bias)>> {
        slf.current_idx += 1;
        let res = slf.items.get(slf.current_idx - 1);
        if res.is_none() {
            return Ok(None);
        }
        let (idxs, bias) = res.unwrap();
        let vars: Vec<_> = idxs
            .iter()
            .map(|&idx| PyVariable::new(VarRef::new(idx, slf.env.clone())))
            .collect();
        let var_obj = match &vars[..] {
            [] => PyConstant().into_py_any(py),
            [a] => PyLinear(a.clone()).into_py_any(py),
            [a, b] => PyQuadratic((a.clone(), b.clone())).into_py_any(py),
            _ => PyHigherOrder(vars).into_py_any(py),
        };
        Ok(Some((var_obj?, *bias)))
    }
}

#[unwindable]
#[pymethods]
impl PyConstant {
    fn __str__(&self) -> String {
        String::from("Constant()")
    }
}

#[unwindable]
#[pymethods]
impl PyLinear {
    #[getter]
    fn get_var(&self) -> PyVariable {
        self.0.clone()
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("var",)
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Linear({})", self.0.name()?))
    }
}

#[unwindable]
#[pymethods]
impl PyQuadratic {
    #[getter]
    fn get_var_a(&self) -> PyVariable {
        self.0 .0.clone()
    }
    #[getter]
    fn get_var_b(&self) -> PyVariable {
        self.0 .1.clone()
    }

    #[classattr]
    fn __match_args__() -> (&'static str, &'static str) {
        ("var_a", "var_b")
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "Quadratic({}, {})",
            self.0 .0.name()?,
            self.0 .1.name()?
        ))
    }
}

#[unwindable]
#[pymethods]
impl PyHigherOrder {
    #[getter]
    fn get_vars(&self) -> Vec<PyVariable> {
        self.0.clone()
    }

    #[classattr]
    fn __match_args__() -> (&'static str,) {
        ("vars",)
    }

    fn __str__(&self) -> PyResult<String> {
        let vnames: Vec<_> = self
            .0
            .iter()
            .map(|x| Ok(x.name()?.clone()))
            .collect::<Result<Vec<String>, VariableNotExistingErr>>()?;
        Ok(format!("HigherOrder({})", vnames.join(", ")))
    }
}

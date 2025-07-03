use super::{
    py_constr::PyConstraint,
    py_env::{PyEnvironment, CURRENT_ENV},
    py_exceptions::NoActiveEnvironmentFoundError,
    py_utilities::Replacement,
    py_var::PyVariable,
};
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
    serialization::{
        Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
    },
};
use either::Either::{self, Left, Right};
use pyo3::exceptions::PyValueError;
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyBytes, IntoPyObjectExt};
use pyo3::{exceptions::PyTypeError, types::PyType};
use std::{cell::RefCell, rc::Rc};

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
    pyclass(unsendable, name = "Expression", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(unsendable, name = "Expression", module = "luna_quantum._core")
)]
#[derive(Clone)]
pub struct PyExpression(pub Either<Expression, Rc<RefCell<Model>>>);

impl PyExpression {
    pub fn new(expr: Expression) -> Self {
        Self(Left(expr))
    }
    pub fn with_parent(parent: Rc<RefCell<Model>>) -> Self {
        Self(Right(parent))
    }

    pub fn get_cloned_expression(&self) -> Expression {
        match &self.0 {
            Left(expr) => expr.clone(),
            Right(parent) => parent.borrow().objective.clone(),
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
    pyclass(unsendable, name = "ExpressionIterator", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(unsendable, name = "ExpressionIterator", module = "luna_quantum._core")
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
    pyclass(unsendable, name = "Constant", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(unsendable, name = "Constant", module = "luna_quantum._core")
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
    pyclass(unsendable, name = "Linear", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(unsendable, name = "Linear", module = "luna_quantum._core")
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
    pyclass(unsendable, name = "Quadratic", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(unsendable, name = "Quadratic", module = "luna_quantum._core")
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
    pyclass(unsendable, name = "HigherOrder", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(unsendable, name = "HigherOrder", module = "luna_quantum._core")
)]
pub struct PyHigherOrder(pub Vec<PyVariable>);

impl PyExpressionIterator {
    fn new(expr: &PyExpression) -> Self {
        Self {
            items: match &expr.0 {
                Left(expr) => expr.items(),
                Right(p) => p.borrow().objective.items(),
            },
            env: match &expr.0 {
                Left(expr) => expr.env.clone(),
                Right(p) => p.borrow().environment.clone(),
            },
            current_idx: 0,
        }
    }
}

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

    /// Get the constant (offset) term in the expression.
    ///
    /// Returns
    /// -------
    /// float
    ///     The constant term.
    fn get_offset(&self) -> f64 {
        match &self.0 {
            Left(expr) => expr.offset(),
            Right(parent) => parent.borrow().objective.offset(),
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
            Right(parent) => parent.borrow().objective.linear(variable.id)?,
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
            Right(parent) => parent.borrow().objective.quadratic(u.id, v.id)?,
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
                .borrow()
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
            Right(parent) => parent.borrow().objective.num_variables(),
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
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        let compress = compress.unwrap_or(level.is_some());
        let base = match &self.0 {
            Left(expr) => expr,
            Right(parent) => &parent.borrow().objective,
        };
        Ok(PyBytes::new(
            py,
            &base.encode().maybe_compress(compress, level)?.versionize(),
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
    fn __add__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: Expression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = match &self.0 {
                Left(e) => e.add(rhs),
                Right(p) => p.borrow().objective.add(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = match &self.0 {
                Left(e) => e.add(rhs.as_ref())?,
                Right(p) => p.borrow().objective.add(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = match (&self.0, rhs.0) {
                (Left(l), Left(r)) => l.add(&r)?,
                (Left(l), Right(r)) => l.add(&r.borrow().objective)?,
                (Right(l), Left(r)) => l.borrow().objective.add(&r)?,
                (Right(l), Right(r)) => l.borrow().objective.add(&r.borrow().objective)?,
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
    fn __radd__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
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
    fn __sub__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: Expression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = match &self.0 {
                Left(e) => e.sub(rhs),
                Right(p) => p.borrow().objective.sub(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = match &self.0 {
                Left(e) => e.sub(rhs.as_ref())?,
                Right(p) => p.borrow().objective.sub(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = match (&self.0, rhs.0) {
                (Left(l), Left(r)) => l.sub(&r)?,
                (Left(l), Right(r)) => l.sub(&r.borrow().objective)?,
                (Right(l), Left(r)) => l.borrow().objective.sub(&r)?,
                (Right(l), Right(r)) => l.borrow().objective.sub(&r.borrow().objective)?,
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
    fn __mul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        let expr: Expression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = match &self.0 {
                Left(e) => e.mul(rhs),
                Right(p) => p.borrow().objective.mul(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = match &self.0 {
                Left(e) => e.mul(rhs.as_ref())?,
                Right(p) => p.borrow().objective.mul(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = match (&self.0, rhs.0) {
                (Left(l), Left(r)) => l.mul(&r)?,
                (Left(l), Right(r)) => l.mul(&r.borrow().objective)?,
                (Right(l), Left(r)) => l.borrow().objective.mul(&r)?,
                (Right(l), Right(r)) => l.borrow().objective.mul(&r.borrow().objective)?,
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
    fn __rmul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
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
            Right(p) => (-&(p.borrow().objective)).sub(other),
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
    pub fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            match &mut self.0 {
                Left(e) => e.add_assign(rhs),
                Right(p) => p.borrow_mut().objective.add_assign(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            match &mut self.0 {
                Left(e) => e.add_assign(rhs.as_ref())?,
                Right(p) => p.borrow_mut().objective.add_assign(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            match (&mut self.0, rhs.0) {
                (Left(l), Left(r)) => l.add_assign(&r)?,
                (Left(l), Right(r)) => l.add_assign(&r.borrow().objective)?,
                (Right(l), Left(r)) => l.borrow_mut().objective.add_assign(&r)?,
                (Right(l), Right(r)) => {
                    l.borrow_mut().objective.add_assign(&r.borrow().objective)?
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
    fn __isub__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            match &mut self.0 {
                Left(e) => e.sub_assign(rhs),
                Right(p) => p.borrow_mut().objective.sub_assign(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            match &mut self.0 {
                Left(e) => e.sub_assign(rhs.as_ref())?,
                Right(p) => p.borrow_mut().objective.sub_assign(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            match (&mut self.0, rhs.0) {
                (Left(l), Left(r)) => l.sub_assign(&r)?,
                (Left(l), Right(r)) => l.sub_assign(&r.borrow().objective)?,
                (Right(l), Left(r)) => l.borrow_mut().objective.sub_assign(&r)?,
                (Right(l), Right(r)) => {
                    l.borrow_mut().objective.sub_assign(&r.borrow().objective)?
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
    fn __imul__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            match &mut self.0 {
                Left(e) => e.mul_assign(rhs),
                Right(p) => p.borrow_mut().objective.mul_assign(rhs),
            }
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            match &mut self.0 {
                Left(e) => e.mul_assign(rhs.as_ref())?,
                Right(p) => p.borrow_mut().objective.mul_assign(rhs.as_ref())?,
            }
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            match (&mut self.0, rhs.0) {
                (Left(l), Left(r)) => l.mul_assign(&r)?,
                (Left(l), Right(r)) => l.mul_assign(&r.borrow().objective)?,
                (Right(l), Left(r)) => l.borrow_mut().objective.mul_assign(&r)?,
                (Right(l), Right(r)) => {
                    l.borrow_mut().objective.mul_assign(&r.borrow().objective)?
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
                    Right(p) => p.borrow().environment.clone(),
                };
                Expression::empty(env).add(1.0)
            }
            1 => match &self.0 {
                Left(expr) => expr.clone(),
                Right(p) => p.borrow().objective.clone(),
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
                    let m = p.borrow();
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
            (Left(l), Right(r)) => *l == r.borrow().objective,
            (Right(l), Left(r)) => l.borrow().objective == *r,
            (Right(l), Right(r)) => l.borrow().objective == r.borrow().objective,
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
    fn __eq__(&self, py: Python, other: PyObject) -> PyResult<PyConstraint> {
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
    fn __le__(&self, py: Python, other: PyObject) -> PyResult<PyConstraint> {
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
    fn __ge__(&self, py: Python, other: PyObject) -> PyResult<PyConstraint> {
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
            Right(p) => -(&p.borrow().objective),
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
    //     *self.borrow() != *other.borrow()
    // }

    fn __str__(&self) -> String {
        match &self.0 {
            Left(expr) => expr.to_string(),
            Right(p) => p.borrow().objective.to_string(),
        }
    }

    fn __repr__(&self) -> String {
        match &self.0 {
            Left(expr) => format!("{:#?}", expr),
            Right(p) => format!("{:#?}", p.borrow().objective),
        }
    }

    /// Get this expression's environment.
    #[getter]
    fn _environment(&self) -> PyEnvironment {
        PyEnvironment(match &self.0 {
            Left(expr) => expr.env.clone(),
            Right(p) => p.borrow().environment.clone(),
        })
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
                Right(other_model) => expr.substitute(&target.0, &other_model.borrow().objective),
            },
            Right(model) => match &replacement.as_expr().0 {
                Left(repl) => (&model.borrow().objective).substitute(&target.0, &repl),
                Right(other_model) => (&model.borrow().objective)
                    .substitute(&target.0, &other_model.borrow().objective),
            },
        }?;
        Ok(PyExpression::new(expr))
    }

    fn equal_contents(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Left(lhs), Left(rhs)) => lhs.is_equal_contents(&rhs),
            (Left(lhs), Right(rhs)) => lhs.is_equal_contents(&rhs.borrow().objective),
            (Right(lhs), Left(rhs)) => lhs.borrow().objective.is_equal_contents(&rhs),
            (Right(lhs), Right(rhs)) => lhs
                .borrow()
                .objective
                .is_equal_contents(&rhs.borrow().objective),
        }
    }
}

#[pymethods]
impl PyExpressionIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>, py: Python) -> PyResult<Option<(PyObject, Bias)>> {
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

#[pymethods]
impl PyConstant {
    fn __str__(&self) -> String {
        String::from("Constant()")
    }
}

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

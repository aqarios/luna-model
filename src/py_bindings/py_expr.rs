use super::{
    py_constr::PyConstraint,
    py_env::{PyEnvironment, CURRENT_ENV},
    py_exceptions::NoActiveEnvironmentFoundError,
    py_var::PyVariable,
};
use crate::core::{
    operations::{
        AddAssignToExpression, AddToExpression, MulAssignToExpression, MulToExpression,
        SubAssignToExpression, SubToExpression,
    },
    Comparator, ConcreteBias, ConcreteExpression, ConcreteIndex, ConcreteMutRcEnvironment,
    ConcreteMutRcExpression, Expression, ExpressionBase, VarRef,
};
use crate::{
    core::expression::ExpressionBaseCreation,
    serialization::{
        Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
    },
};
use derive_more::{Deref, DerefMut};
use pyo3::exceptions::PyValueError;
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyBytes, IntoPyObjectExt};
use pyo3::{exceptions::PyTypeError, types::PyType};
use std::cell::Ref;
use std::{ops::Deref, rc::Rc};

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
#[pyclass(unsendable, name = "Expression", module = "aqmodels")]
#[derive(Deref, DerefMut, Clone)]
pub struct PyExpression(pub ConcreteMutRcExpression);

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
#[pyclass(unsendable, name = "ExpressionIterator", module = "aqmodels")]
pub struct PyExpressionIterator {
    items: Vec<(Vec<ConcreteIndex>, ConcreteBias)>,
    env: ConcreteMutRcEnvironment,
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
#[pyclass(unsendable, name = "Constant", module = "aqmodels")]
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
#[pyclass(unsendable, name = "Linear", module = "aqmodels")]
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
#[pyclass(unsendable, name = "Quadratic", module = "aqmodels")]
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
#[pyclass(unsendable, name = "HigherOrder", module = "aqmodels")]
pub struct PyHigherOrder(pub Vec<PyVariable>);

impl PyExpression {
    pub fn new(expression: ConcreteExpression) -> Self {
        Self(expression.into())
    }
}

impl PyExpressionIterator {
    fn new(expr: Ref<ConcreteExpression>) -> Self {
        Self {
            items: expr.items(),
            env: Rc::clone(&expr.env),
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
    fn py_new(env: Option<&mut PyEnvironment>) -> PyResult<Self> {
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
        self.borrow().offset()
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
        Ok(self.borrow().linear(variable.id)?)
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
        Ok(self.borrow().quadratic(u.id, v.id)?)
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
        Ok(self
            .borrow()
            .higher_order(&variables.iter().map(|v| v.id).collect())?)
    }

    /// Return the number of distinct variables in the expression.
    ///
    /// Returns
    /// -------
    /// int
    ///     Number of variables with non-zero coefficients.
    #[getter]
    fn get_num_variables(&self) -> usize {
        self.borrow().num_variables()
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
        Ok(PyBytes::new(
            py,
            &self
                .borrow()
                .deref()
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
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.borrow().add(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.borrow().add(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = self.borrow().add(rhs.borrow().deref())?;
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
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.borrow().sub(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.borrow().sub(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = self.borrow().sub(rhs.borrow().deref())?;
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
        let expr: ConcreteExpression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.borrow().mul(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.borrow().mul(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = self.borrow().mul(rhs.borrow().deref())?;
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
    fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            self.borrow_mut().add_assign(rhs)
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            self.borrow_mut().add_assign(rhs.as_ref())?
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            self.borrow_mut().add_assign(rhs.borrow().deref())?
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
            self.borrow_mut().sub_assign(rhs)
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            self.borrow_mut().sub_assign(rhs.as_ref())?
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            self.borrow_mut().sub_assign(rhs.borrow().deref())?
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
            self.borrow_mut().mul_assign(rhs)
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            self.borrow_mut().mul_assign(rhs.as_ref())?
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            self.borrow_mut().mul_assign(rhs.borrow().deref())?
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
            0 => Expression::empty(Rc::clone(&self.borrow().env)).add(1.0),
            1 => self.0.borrow().deref().clone(),
            _ => {
                let mut base = Expression::empty(Rc::clone(&self.borrow().env)).add(1.0);
                for _ in 0..other {
                    base.mul_assign(self.borrow().deref())?;
                }
                base
            }
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
        *self.borrow() == *other.borrow()
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
        PyExpression::new(-self.borrow().deref())
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
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.borrow())
    }

    /// Get this expression's environment.
    #[getter]
    fn _environment(&self) -> PyEnvironment {
        PyEnvironment(self.0.borrow().env.clone())
    }

    /// Iterate over the single components of an expression. An *component* refers to
    /// a single constant, linear, quadratic, or higher-order term of an expression.
    //
    /// Returns
    /// -------
    /// ExpressionIterator
    ///     The iterator over the expression's components.
    fn items(&self) -> PyExpressionIterator {
        PyExpressionIterator::new(self.borrow())
    }
}

#[pymethods]
impl PyExpressionIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(
        mut slf: PyRefMut<'_, Self>,
        py: Python,
    ) -> PyResult<Option<(PyObject, ConcreteBias)>> {
        slf.current_idx += 1;
        let res = slf.items.get(slf.current_idx - 1);
        if res.is_none() {
            return Ok(None);
        }
        let (idxs, bias) = res.unwrap();
        let vars: Vec<_> = idxs
            .iter()
            .map(|&idx| PyVariable::new(VarRef::new(idx, Rc::clone(&slf.env))))
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

    fn __str__(&self) -> String {
        format!("Linear({})", self.0.name())
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

    fn __str__(&self) -> String {
        format!("Quadratic({}, {})", self.0 .0.name(), self.0 .1.name())
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

    fn __str__(&self) -> String {
        let vnames: Vec<_> = self.0.iter().map(|x| x.name().clone()).collect();
        format!("HigherOrder({})", vnames.join(", "))
    }
}

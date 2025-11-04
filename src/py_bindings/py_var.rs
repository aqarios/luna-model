use super::py_constr::PyConstraint;
use super::py_env::{PyEnvironment, CURRENT_ENV};
use super::py_exceptions::NoActiveEnvironmentFoundError;
use super::unwind;
use super::{py_bounds::PyBounds, py_expr::PyExpression};
use crate::core::expression::ExpressionBaseCreation;
use crate::core::operations::{
    AddToExpression, MulAssignToExpression, MulToExpression, NegToExpression, RSubToExpression,
    SubAssignToExpression, SubToExpression,
};
use crate::core::{Comparator, Constraint, Expression, VarRef, Vtype};
use crate::errors::VariableNotExistingErr;
use crate::utils::Share;
use derive_more::{Deref, DerefMut};
use either::Either::{Left, Right};
use pyo3::exceptions::{PyRuntimeError, PyTypeError, PyValueError};
use pyo3::types::PyBool;
use pyo3::{prelude::*, IntoPyObjectExt};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Not;
use unwind_macros::unwindable;

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
/// >>> from luna_model import Variable, Environment, Vtype, Bounds
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
#[pyclass(subclass, name = "Variable", module = "luna_model._core")]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyVariable(pub Share<VarRef>);

impl PyVariable {
    pub fn new(varref: VarRef) -> Self {
        Self(varref.into())
    }

    pub fn name(&self) -> Result<String, VariableNotExistingErr> {
        Ok(self.env.access().get_for_index(self.id)?.name.clone())
    }

    pub fn bounds(&self) -> Result<PyBounds, VariableNotExistingErr> {
        Ok(PyBounds(
            self.env.access().get_for_index(self.id)?.bounds.into(),
        ))
    }

    pub fn vtype(&self) -> Result<Vtype, VariableNotExistingErr> {
        Ok(self.env.access().get_for_index(self.id)?.vtype)
    }

    pub fn hash<H: Hasher>(&self, state: &mut H) -> Result<(), VariableNotExistingErr> {
        Ok(self.name()?.hash(state))
    }
}

impl PartialEq<Self> for PyVariable {
    fn eq(&self, other: &Self) -> bool {
        let self_idx: usize = self.id.into();
        let other_idx: usize = other.id.into();
        if self.env.ptr_eq(&other.env) {
            // euqal pointer so also equal id, now let's check the
            // variable itself.
            let env = self.env.access();
            let lhs = &env[self_idx];
            let rhs = &env[other_idx];
            lhs == rhs
        } else {
            self.env.access().id() == other.env.access().id()
                && self.env.access()[self_idx] == other.env.access()[other_idx]
        }
    }
}

impl Eq for PyVariable {}

#[unwindable]
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

        Ok(PyVariable::new(env.0.add_variable(
            &name,
            vtype,
            bounds.map(|pb| pb.into()),
        )?))
    }

    #[getter]
    pub fn get_id(&self) -> usize {
        self.id.0 as usize
    }

    /// Get the name of the variable.
    #[getter]
    fn get_name(&self) -> PyResult<String> {
        Ok(self.name()?)
    }

    /// Get the bounds of the variable.
    #[getter]
    fn get_bounds(&self) -> PyResult<PyBounds> {
        Ok(self.bounds()?)
    }

    #[getter]
    fn get_vtype(&self) -> PyResult<Vtype> {
        Ok(self.vtype()?)
    }

    /// Compute the hash of the variable.
    fn __hash__(&self) -> PyResult<u64> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s)?;
        Ok(s.finish())
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
    fn __add__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyExpression> {
        self.check_living()?;
        let expr: Expression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.add(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.add(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = match &rhs.0 {
                Left(e) => e.add(self.as_ref())?,
                Right(p) => p.access().objective.add(self.as_ref())?,
            };
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
    fn __radd__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyExpression> {
        self.check_living()?;
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
    fn __sub__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyExpression> {
        self.check_living()?;
        let expr: Expression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.add(-rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.sub(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = match &rhs.0 {
                Left(e) => (e.mul(-1.0)).add(self.as_ref())?,
                Right(p) => (p.access().objective.mul(-1.0)).add(self.as_ref())?,
            };
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
    fn __rsub__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyExpression> {
        self.check_living()?;
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
    fn __mul__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyExpression> {
        self.check_living()?;
        let expr: Expression;
        if let Ok(rhs) = other.extract::<f64>(py) {
            expr = self.mul(rhs);
        } else if let Ok(rhs) = other.extract::<PyVariable>(py) {
            expr = self.mul(rhs.as_ref())?;
        } else if let Ok(rhs) = other.extract::<PyExpression>(py) {
            expr = match &rhs.0 {
                Left(e) => e.mul(self.as_ref())?,
                Right(p) => p.access().objective.mul(self.as_ref())?,
            };
        } else {
            return Err(PyTypeError::new_err("unsupported type for operation"));
        }

        Ok(PyExpression::new(expr))
    }

    /// Invert this variable. This operation is only supported on
    /// Binary variables. For all other variable types it raises the
    /// `UnsupportedOperationError`.
    ///
    /// Returns
    /// -------
    /// Expression
    ///     The resulting symbolic expression.
    ///
    /// Raises
    /// ------
    /// UnsupportedOperationErr
    ///     If the operand is a variable of any type other than `Binary`.
    fn not(&self) -> PyResult<PyVariable> {
        self.check_living()?;
        Ok(PyVariable::new(self.0.not()?))
    }

    fn __invert__(&self) -> PyResult<PyVariable> {
        self.check_living()?;
        Ok(PyVariable::new(self.0.not()?))
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
    fn __rmul__(&self, py: Python, other: Py<PyAny>) -> PyResult<PyExpression> {
        self.check_living()?;
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
    fn __pow__(&self, other: isize, modparam: Option<isize>) -> PyResult<PyExpression> {
        self.check_living()?;
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
            0 => Expression::empty(self.env.clone()).add(1.0),
            1 => Expression::new_linear_single(self.env.clone(), self.id, 1.0),
            2 => Expression::new_quadratic(self.env.clone(), self.id, self.id, 1.0),
            _ => {
                let mut base = Expression::new_linear_single(self.env.clone(), self.id, 1.0);
                for _ in 1..other.into() {
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
    fn __neg__(&self) -> PyResult<PyExpression> {
        self.check_living()?;
        Ok(PyExpression::new(self.0.neg()))
    }

    /// Either creates an constraint: variable == int | float | Expression or computes
    /// the equality of two variables: variable == variable.
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
    /// Constraint | bool
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If the right-hand side is not of type float, int, Variable or Expression.
    fn __eq__(&self, py: Python, rhs: Py<PyAny>) -> PyResult<Py<PyAny>> {
        self.check_living()?;
        if let Ok(var) = rhs.extract::<PyVariable>(py) {
            Ok(PyBool::new(py, *self == var).to_owned().into_py_any(py)?)
        } else {
            #[allow(deprecated)]
            self.make_constraint(py, rhs, Comparator::Eq)
                .map(|c| c.into_py_any(py).unwrap())
            // todo(team): handle unwrap here better.
        }
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
    fn __le__(&self, py: Python, rhs: Py<PyAny>) -> PyResult<PyConstraint> {
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
    fn __ge__(&self, py: Python, rhs: Py<PyAny>) -> PyResult<PyConstraint> {
        self.make_constraint(py, rhs, Comparator::Ge)
    }

    fn __str__(&self) -> PyResult<String> {
        self.check_living()?;
        Ok(self.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.check_living()?;
        Ok(format!("{:#?}", self.0))
    }

    /// Get this variables's environment.
    #[getter]
    fn _environment(&self) -> PyResult<PyEnvironment> {
        self.check_living()?;
        Ok(PyEnvironment(self.env.clone()))
    }
}

impl PyVariable {
    fn make_constraint(
        &self,
        py: Python,
        rhs: Py<PyAny>,
        comparator: Comparator,
    ) -> PyResult<PyConstraint> {
        self.check_living()?;
        let mut lhs = Expression::new_linear_single(self.env.clone(), self.id, 1.0);
        let bias: PyResult<f64> = if let Ok(bias) = rhs.extract::<f64>(py) {
            Ok(bias)
        } else if let Ok(var) = rhs.extract::<PyVariable>(py) {
            lhs.sub_assign(var.as_ref())?;
            Ok(0.0)
        } else if let Ok(expr) = rhs.extract::<PyExpression>(py) {
            match &expr.0 {
                Left(e) => lhs.sub_assign(e)?,
                Right(p) => lhs.sub_assign(&p.access().objective)?,
            };
            Ok(0.0)
        } else {
            Err(PyTypeError::new_err("unsupported type for operation"))
        };
        Ok(PyConstraint::new(Constraint::new(
            lhs, bias?, comparator, None,
        )?))
    }
}

#[unwindable]
#[pymethods]
impl Vtype {
    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }

    #[getter]
    fn get_name(&self) -> String {
        match &self {
            Self::Binary => String::from("Binary"),
            Self::InvertedBinary=> String::from("InvertedBinary"),
            Self::Spin => String::from("Spin"),
            Self::Integer => String::from("Integer"),
            Self::Real => String::from("Real"),
            Self::__Ghost => {
                panic!("you should not be able to interact with __Ghost variables in Python.")
            }
        }
    }
    #[getter]
    fn get_value(&self) -> PyResult<String> {
        self.get_name()
    }
}

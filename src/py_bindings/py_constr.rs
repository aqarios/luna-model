use super::unwind;
use super::{py_env::PyEnvironment, py_expr::PyExpression, py_var::PyVariable};
use crate::utils::ShareMut;
use crate::{
    core::{
        expression::ExpressionBaseCreation, operations::SubAssignToExpression, Comparator,
        Constraint, ConstraintKey, Constraints, ContentEquality, Expression, Model,
    },
    serialization::{Decodable, Decompressable, Encodable, Unversionizable},
};
use derive_more::{Deref, DerefMut};
use either::Either::{self, Left, Right};
use pyo3::{exceptions::PyTypeError, types::PyType};
use pyo3::{prelude::*, types::PyBytes};
use unwind_macros::unwindable;

/// A collection of symbolic constraints used to define a model.
///
/// The `Constraints` object serves as a container for individual `Constraint`
/// instances. It supports adding constraints programmatically and exporting
/// them for serialization.
///
/// Constraints are typically added using `add_constraint()` or the `+=` operator.
///
/// Examples
/// --------
/// >>> from luna_quantum import Constraints, Constraint, Environment, Variable
/// >>> with Environment():
/// ...     x = Variable("x")
/// ...     c = Constraint(x + 1, 0.0, Comparator.Le)
///
/// >>> cs = Constraints()
/// >>> cs.add_constraint(c)
///
/// >>> cs += x >= 1.0
///
/// Serialization:
///
/// >>> blob = cs.encode()
/// >>> expr = Constraints.decode(blob)
///
/// Notes
/// -----
/// - This class does not check feasibility or enforce satisfaction.
/// - Use `encode()`/`decode()` to serialize constraints alongside expressions.
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "Constraints", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "Constraints", module = "luna_quantum._core")
)]
#[derive(Debug, Clone)]
pub struct PyConstraints {
    pub data: Either<Constraints, ShareMut<Model>>,
}

impl PyConstraints {
    pub fn new(constrs: Constraints) -> Self {
        // Self(Arc::new(Mutex::new(constrs)))
        Self {
            data: Left(constrs),
        }
    }

    pub fn with_parent(parent: ShareMut<Model>) -> Self {
        Self {
            data: Right(parent),
        }
    }

    pub fn get_cloned_constraints(&self) -> Constraints {
        match &self.data {
            Left(constrs) => constrs.clone(),
            Right(parent) => parent.access().constraints.clone(),
        }
    }
}

/// A symbolic constraint formed by comparing an expression to a constant.
///
/// A `Constraint` captures a relation of the form:
/// `expression comparator constant`, where the comparator is one of:
/// `==`, `<=`, or `>=`.
///
/// While constraints are usually created by comparing an `Expression` to a scalar
/// (e.g., `expr == 3.0`), they can also be constructed manually using this class.
///
/// Parameters
/// ----------
/// lhs : Expression
///     The left-hand side expression.
/// rhs : float
///     The scalar right-hand side value.
/// comparator : Comparator
///     The relation between lhs and rhs (e.g., `Comparator.Eq`).
///
/// Examples
/// --------
/// >>> from luna_quantum import Environment, Variable, Constraint, Comparator
/// >>> with Environment():
/// ...     x = Variable("x")
/// ...     c = Constraint(x + 2, 5.0, Comparator.Eq)
///
/// Or create via comparison:
///
/// >>> expr = 2 * x + 1
/// >>> c2 = expr <= 10.0
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "Constraint", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "Constraint", module = "luna_quantum._core")
)]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyConstraint(pub ShareMut<Constraint>);

impl PyConstraint {
    pub fn new(constraint: Constraint) -> Self {
        Self(ShareMut::new(constraint))
    }

    pub fn new_py(
        py: Python,
        lhs: &PyExpression,
        rhs: PyObject,
        comparator: Comparator,
    ) -> PyResult<PyConstraint> {
        let mut lhs = lhs.clone();
        let bias: PyResult<f64> = if let Ok(bias) = rhs.extract::<f64>(py) {
            Ok(bias)
        } else if let Ok(var) = rhs.extract::<PyVariable>(py) {
            match &mut lhs.0 {
                Left(expr) => expr.sub_assign(var.as_ref())?,
                Right(parent) => parent.access_mut().objective.sub_assign(var.as_ref())?,
            };
            Ok(0.0)
        } else if let Ok(expr) = rhs.extract::<PyExpression>(py) {
            match (&mut lhs.0, &expr.0) {
                (Left(l), Left(r)) => l.sub_assign(r)?,
                (Left(l), Right(r)) => l.sub_assign(&r.access().objective)?,
                (Right(l), Left(r)) => l.access_mut().objective.sub_assign(r)?,
                (Right(l), Right(r)) => {
                    l.access_mut().objective.sub_assign(&r.access().objective)?
                }
            }
            Ok(0.0)
        } else {
            Err(PyTypeError::new_err("unsupported type for operation"))
        };
        Ok(PyConstraint::new(Constraint::new(
            match &lhs.0 {
                Left(expr) => expr.clone(),
                Right(parent) => parent.access().objective.clone(),
            },
            bias?,
            comparator,
            None,
        )?))
    }
}

#[unwindable]
#[pymethods]
impl PyConstraint {
    /// Construct a new symbolic constraint.
    ///
    /// Parameters
    /// ----------
    /// lhs : Expression | Variable
    ///     Left-hand side symbolic expression or variable.
    /// rhs : int | float | Expression | Variable
    ///     Scalar right-hand side constant.
    /// comparator : Comparator
    ///     Relational operator (e.g., Comparator.Eq, Comparator.Le).
    /// name : str
    ///     The name of the constraint
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If lhs is not an Expression or rhs is not a scalar float.
    /// IllegalConstraintNameError
    ///     If the constraint is tried to be created with an illegal name.
    #[new]
    #[pyo3(signature=(lhs, rhs, comparator, name=None))]
    fn py_new(
        py: Python,
        lhs: PyObject,
        rhs: PyObject,
        comparator: Comparator,
        name: Option<String>,
    ) -> PyResult<Self> {
        let lhs: PyResult<Expression> = if let Ok(expr) = lhs.extract::<PyExpression>(py) {
            Ok(match &expr.0 {
                Left(e) => e.clone(),
                Right(parent) => parent.access().objective.clone(),
            })
        } else if let Ok(var) = lhs.extract::<PyVariable>(py) {
            Ok(Expression::new_linear_single(var.env.clone(), var.id, 1.0))
        } else {
            Err(PyTypeError::new_err(
                "unsupported type for lhs in operation",
            ))
        };
        let mut lhs = lhs?;
        let bias = if let Ok(bias) = rhs.extract::<f64>(py) {
            Ok(bias)
        } else if let Ok(var) = rhs.extract::<PyVariable>(py) {
            lhs.sub_assign(var.as_ref())?;
            Ok(0.0)
        } else if let Ok(expr) = rhs.extract::<PyExpression>(py) {
            match &expr.0 {
                Left(e) => lhs.sub_assign(e)?,
                Right(parent) => lhs.sub_assign(&parent.access().objective)?,
            }
            Ok(0.0)
        } else {
            Err(PyTypeError::new_err(
                "unsupported type for rhs in operation",
            ))
        };
        Ok(PyConstraint::new(Constraint::new(
            lhs, bias?, comparator, name,
        )?))
    }

    fn __eq__(&self, other: Self) -> bool {
        if self.ptr_eq(&other) {
            true
        } else {
            *self.access() == *other.access()
        }
    }

    fn __str__(&self) -> String {
        self.access().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.access())
    }

    /// Get the name of the constraint.
    ///
    /// Returns
    /// -------
    /// str, optional
    ///     Returns the name of the constraint as a string or None if it is unnamed.
    #[getter]
    fn name(&self) -> Option<String> {
        self.access().name.clone()
    }

    /// Get the left-hand side of the constraint
    ///
    /// Returns
    /// -------
    /// Expression
    ///     The left-hand side expression.
    #[getter]
    fn lhs(&self) -> PyExpression {
        // PyExpression(Arc::new(Mutex::new(self.access().lhs.clone())))
        PyExpression::new(self.access().lhs.clone())
    }

    /// Get the right-hand side of the constraint
    ///
    /// Returns
    /// -------
    /// float
    ///     The right-hand side expression.
    #[getter]
    fn rhs(&self) -> f64 {
        self.access().rhs
    }

    /// Get the comparator of the constraint
    ///
    /// Returns
    /// -------
    /// Comparator
    ///     The comparator of the constraint.
    #[getter]
    fn comparator(&self) -> Comparator {
        self.access().comparator
    }
}

#[unwindable]
#[pymethods]
impl PyConstraints {
    #[new]
    fn py_new() -> Self {
        PyConstraints::new(Constraints::default())
    }

    /// In-place constraint addition using `+=`.
    ///
    /// Parameters
    /// ----------
    /// constraint : Constraint | tuple[Constraint, str]
    ///     The constraint to add.
    ///
    /// Returns
    /// -------
    /// Constraints
    ///     The updated collection.
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If the value is not a `Constraint` or valid symbolic comparison.
    fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok((constr, name)) = other.extract::<(PyConstraint, String)>(py) {
            Ok(self.add_constraint(constr, Some(name))?)
        } else if let Ok(constr) = other.extract::<PyConstraint>(py) {
            Ok(self.add_constraint(constr, None)?)
        } else {
            Err(PyTypeError::new_err("unsupported type for operation"))
        }
    }

    /// Add a constraint to the collection.
    ///
    /// Parameters
    /// ----------
    /// constraint : Constraint
    ///     The constraint to be added.
    /// name : str, optional
    ///     The name of the constraint to be added.
    #[pyo3(signature=(constraint, name=None))]
    fn add_constraint(&mut self, constraint: PyConstraint, name: Option<String>) -> PyResult<()> {
        constraint.access_mut().set_name(name)?;
        match &mut self.data {
            Left(constrs) => constrs.add_assign(&constraint.access())?,
            Right(parent) => {
                parent
                    .access_mut()
                    .constraints
                    .add_assign(&constraint.access())?;
            }
        }
        Ok(())
    }

    fn __getitem__(&self, n: ConstraintKey) -> PyResult<PyConstraint> {
        let constr = match &self.data {
            Left(constrs) => constrs.get_constraint(n)?.clone(),
            Right(parent) => parent.access().constraints.get_constraint(n)?.clone(),
        };
        Ok(PyConstraint::new(constr))
    }

    fn __setitem__(&mut self, n: ConstraintKey, c: PyConstraint) -> PyResult<()> {
        let constraint = c.access().clone();
        match &mut self.data {
            Left(constrs) => constrs.set_constraint(n, constraint)?,
            Right(parent) => parent
                .access_mut()
                .constraints
                .set_constraint(n, constraint)?,
        };
        Ok(())
    }

    fn __len__(&self) -> usize {
        match &self.data {
            Left(constrs) => constrs.len(),
            Right(parent) => parent.access().constraints.len(),
        }
    }

    fn __eq__(&self, other: Self) -> bool {
        match (&self.data, &other.data) {
            (Left(lhs), Left(rhs)) => lhs == rhs,
            (Left(lhs), Right(rhs)) => *lhs == rhs.access().constraints,
            (Right(lhs), Left(rhs)) => lhs.access().constraints == *rhs,
            (Right(lhs), Right(rhs)) => {
                if lhs.ptr_eq(rhs) {
                    true
                } else {
                    lhs.access().constraints == rhs.access().constraints
                }
            }
        }
    }

    fn __str__(&self) -> String {
        match &self.data {
            Left(constrs) => constrs.to_string(),
            Right(parent) => parent.access().constraints.to_string(),
        }
    }

    fn __repr__(&self) -> String {
        let r = match &self.data {
            Left(constrs) => constrs.to_string(),
            Right(parent) => parent.access().constraints.to_string(),
        };
        format!("{:#?}", r)
    }

    /// Serialize the constraint collection to a binary blob.
    ///
    /// Parameters
    /// ----------
    /// compress : bool, optional
    ///     Whether to compress the result. Default is True.
    /// level : int, optional
    ///     Compression level (0–9). Default is 3.
    ///
    /// Returns
    /// -------
    /// bytes
    ///     Encoded representation of the constraints.
    ///
    /// Raises
    /// ------
    /// IOError
    ///     If serialization fails.
    #[pyo3(signature=(compress=true, level=3))]
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        match &self.data {
            Left(constrs) => Ok(PyBytes::new(py, &constrs.encode(compress, level)?).into()),
            Right(parent) => {
                Ok(PyBytes::new(py, &parent.access().constraints.encode(compress, level)?).into())
            }
        }
    }

    /// Alias for `encode()`.
    ///
    /// See `encode()` for details.
    #[pyo3(signature=(compress=true, level=3))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<PyObject> {
        self.encode(py, compress, level)
    }

    /// Deserialize an expression from binary constraint data.
    ///
    /// Parameters
    /// ----------
    /// data : bytes
    ///     Encoded blob from `encode()`.
    ///
    /// Returns
    /// -------
    /// Expression
    ///     Expression reconstructed from the constraint context.
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
        Ok(PyConstraints::new(
            data.as_bytes(py)
                .unversionize()
                .decompress()?
                .decode(env.0)?,
        ))
    }

    /// Alias for `decode()`.
    ///
    /// See `decode()` for usage.
    #[classmethod]
    fn deserialize(
        cls: &Bound<'_, PyType>,
        py: Python,
        data: Py<PyBytes>,
        env: PyEnvironment,
    ) -> PyResult<Self> {
        Self::decode(cls, py, data, env)
    }

    fn equal_contents(&self, other: &Self) -> bool {
        match (&self.data, &other.data) {
            (Left(lhs), Left(rhs)) => lhs.is_equal_contents(&rhs),
            (Left(lhs), Right(rhs)) => lhs.is_equal_contents(&rhs.access().constraints),
            (Right(lhs), Left(rhs)) => lhs.access().constraints.is_equal_contents(&rhs),
            (Right(lhs), Right(rhs)) => lhs
                .access()
                .constraints
                .is_equal_contents(&rhs.access().constraints),
        }
    }

    fn get(&self, item: ConstraintKey) -> PyResult<PyConstraint> {
        let constr = match &self.data {
            Left(d) => d.get_constraint(item)?.clone(),
            Right(d) => d.access().constraints.get_constraint(item)?.clone(),
        };
        Ok(PyConstraint::new(constr))
    }

    fn remove(&mut self, item: ConstraintKey) {
        match &mut self.data {
            Left(d) => d.remove_constraint(item),
            Right(d) => d.access_mut().constraints.remove_constraint(item),
        }
    }

    /// Get all unique constraint types identified using their comparator.
    #[pyo3(name = "ctypes")]
    fn get_ctypes(&self) -> Vec<Comparator> {
        match &self.data {
            Left(d) => d.ctypes(),
            Right(d) => d.access().constraints.ctypes(),
        }
    }
}

#[unwindable]
#[pymethods]
impl Comparator {
    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }

    #[getter]
    fn get_name(&self) -> String {
        match &self {
            Self::Eq => String::from("Eq"),
            Self::Le => String::from("Le"),
            Self::Ge => String::from("Ge"),
        }
    }
    #[getter]
    fn get_value(&self) -> PyResult<String> {
        self.get_name()
    }
}

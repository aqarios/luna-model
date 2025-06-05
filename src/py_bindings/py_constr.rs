use std::{
    cell::RefCell,
    ops::{AddAssign, Deref},
    rc::Rc,
};

use crate::{
    core::{
        expression::ExpressionBaseCreation, operations::SubAssignToExpression, Comparator,
        ConcreteConstraint, ConcreteConstraints, ConcreteExpression, ConcreteMutRcConstraint,
        ConcreteMutRcConstraints, Constraint, Create, Expression,
    },
    serialization::{
        Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
    },
};
use derive_more::{Deref, DerefMut};
use pyo3::{exceptions::PyTypeError, types::PyType};
use pyo3::{prelude::*, types::PyBytes};

use super::{py_env::PyEnvironment, py_expr::PyExpression, py_var::PyVariable};

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
#[pyclass(unsendable, name = "Constraints", module = "aqmodels")]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyConstraints(pub ConcreteMutRcConstraints);

impl PyConstraints {
    pub fn new(constrs: ConcreteConstraints) -> Self {
        Self(constrs.into())
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
#[pyclass(unsendable, name = "Constraint", module = "aqmodels")]
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PyConstraint(pub ConcreteMutRcConstraint);

impl PyConstraint {
    pub fn new(constraint: ConcreteConstraint) -> Self {
        Self(constraint.into())
    }

    pub fn new_py(
        py: Python,
        lhs: &PyExpression,
        rhs: PyObject,
        comparator: Comparator,
    ) -> PyResult<PyConstraint> {
        let mut lhs = lhs.borrow().deref().clone();
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
        Ok(PyConstraint::new(Constraint::new(
            Rc::new(RefCell::new(lhs)),
            bias?,
            comparator,
            None,
        )?))
    }
}

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
        let lhs: PyResult<ConcreteExpression> = if let Ok(expr) = lhs.extract::<PyExpression>(py) {
            Ok(expr.0.borrow().deref().clone())
        } else if let Ok(var) = lhs.extract::<PyVariable>(py) {
            Ok(Expression::new_linear_single(
                Rc::clone(&var.env),
                var.id,
                1.0,
            ))
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
            lhs.sub_assign(expr.borrow().deref())?;
            Ok(0.0)
        } else {
            Err(PyTypeError::new_err(
                "unsupported type for rhs in operation",
            ))
        };
        Ok(PyConstraint::new(ConcreteConstraint::new(
            Rc::new(RefCell::new(lhs)),
            bias?,
            comparator,
            name,
        )?))
    }

    fn __eq__(&self, other: Self) -> bool {
        *self.borrow() == *other.borrow()
    }

    fn __str__(&self) -> String {
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.borrow())
    }

    /// Get the name of the constraint.
    ///
    /// Returns
    /// -------
    /// str, optional
    ///     Returns the name of the constraint as a string or None if it is unnamed.
    #[getter]
    fn name(&self) -> Option<String> {
        self.borrow().name.clone()
    }

    /// Get the left-hand side of the constraint
    ///
    /// Returns
    /// -------
    /// Expression
    ///     The left-hand side expression.
    #[getter]
    fn lhs(&self) -> PyExpression {
        PyExpression(Rc::clone(&self.borrow().lhs))
    }

    /// Get the right-hand side of the constraint
    ///
    /// Returns
    /// -------
    /// float
    ///     The right-hand side expression.
    #[getter]
    fn rhs(&self) -> f64 {
        self.borrow().rhs
    }

    /// Get the comparator of the constraint
    ///
    /// Returns
    /// -------
    /// Comparator
    ///     The comparator of the constraint.
    #[getter]
    fn comparator(&self) -> Comparator {
        self.borrow().comparator
    }
}

#[pymethods]
impl PyConstraints {
    #[new]
    fn py_new() -> Self {
        PyConstraints(ConcreteMutRcConstraints::create())
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
        constraint.borrow_mut().set_name(name)?;
        self.borrow_mut().add_assign(constraint.borrow().deref());
        Ok(())
    }

    fn __getitem__(&self, n: usize) -> PyResult<PyConstraint> {
        // todo: can we remove the clone here? acceptable for now. Make it more like
        // a view.
        Ok(PyConstraint::new(self.borrow().get_constraint(n)?.clone()))
    }
    
    fn __len__(&self) -> usize {
        self.borrow().constraints.len()
    }

    fn __eq__(&self, other: Self) -> bool {
        *self.borrow() == *other.borrow()
    }

    fn __str__(&self) -> String {
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.borrow())
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
}

#[pymethods]
impl Comparator {
    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}

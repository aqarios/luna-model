use lunamodel_core::{Expression, ops::LmAddAssign};
use lunamodel_unwind::*;
use pyo3::prelude::*;

use crate::{PyEnvironment, PyExpression};

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
    pub fn py_new(env: Option<PyEnvironment>) -> PyResult<Self> {
        let penv: PyEnvironment = env.try_into()?;
        Ok(PyExpression::new(Expression::empty(penv.env)))
    }

    #[staticmethod]
    #[pyo3(name="const", signature=(val, env=None))]
    pub fn constant(val: f64, env: Option<PyEnvironment>) -> PyResult<Self> {
        let pyenv: PyEnvironment = env.try_into()?;
        let mut expr = Expression::empty(pyenv.env);
        expr.add_assign(val)?;
        Ok(expr.into())
    }
}

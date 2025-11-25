use pyo3::prelude::*;

use crate::{
    PyEnvironment, PyExpression,
    environment::{ACTIVE_ENV, get_active_env},
};

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
        let penv: PyEnvironment = env.into();
        Ok(PyExpression::new(Expression::empty(penv.env)))
    }

    #[staticmethod]
    #[pyo3(name="const", signature=(val, env=None))]
    pub fn constant(val: f64, env: Option<PyEnvironment>) -> PyResult<Self> {
        let penv: PyEnvironment = env.into();
        Ok(PyExpression::new(Expression::constant(penv.env, val)))
    }
}

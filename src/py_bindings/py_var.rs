use super::py_env::PyEnvironment;
use super::py_vtype::PyVtype;
use super::{py_bounds::PyBounds, py_expr::PyExpression};
use crate::core::{
    environment, Expression, ExpressionBaseInternal, VarRef, VariableExistsException,
};

use derive_more::{Deref, DerefMut};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyclass(unsendable, subclass, name = "Variable")]
#[derive(Deref, DerefMut)]
pub struct PyVariable(VarRef);

#[pymethods]
impl PyVariable {
    #[new]
    #[pyo3(signature=(name, env, vtype=None, bounds=None))]
    fn new(
        name: String,
        env: &mut PyEnvironment,
        vtype: Option<PyVtype>,
        bounds: Option<PyBounds>,
    ) -> PyResult<Self> {
        environment::add_varibale(env.clone(), &name, vtype.as_deref(), bounds.as_deref())
            .map(PyVariable)
            .map_err(|e| VariableExistsException::new_err(format!("{}: {}", e.to_string(), name)))
    }

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        if let Ok(rhs) = other.extract::<f64>(py) {
            Ok(PyExpression(Expression::new_from_weighted_variable(
                self.env.clone(),
                self.id,
                rhs,
            )))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }
}

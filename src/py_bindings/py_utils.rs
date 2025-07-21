use super::{
    py_env::{PyEnvironment, CURRENT_ENV},
    py_exceptions::StartCannotBeInferredError,
    py_expr::PyExpression,
    py_var::PyVariable,
};
use crate::core::{expression::ExpressionBaseCreation, Expression, SharedEnvironment};
use either::Either::{Left, Right};
use pyo3::{
    prelude::{pyfunction, Bound, PyAny},
    types::PyAnyMethods,
    PyResult, Python,
};

#[pyfunction]
#[pyo3(signature=(iterable, start=None))]
pub fn quicksum(
    py: Python,
    iterable: &Bound<PyAny>,
    start: Option<PyExpression>,
) -> PyResult<PyExpression> {
    let items: Vec<_> = iterable.try_iter()?.collect();

    let start: PyResult<PyExpression> = if let Some(s) = start {
        Ok(s)
    } else {
        let env: Option<PyEnvironment> = CURRENT_ENV.with(|current| current.borrow().clone());
        match env {
            Some(env) => Ok(PyExpression::new(Expression::empty(env.0))),
            None => {
                let mut env: Option<SharedEnvironment> = None;
                for item in items.iter() {
                    let obj = match item {
                        Ok(e) => e,
                        Err(err) => return Err(err.clone_ref(py)),
                    };
                    if let Ok(rhs) = obj.extract::<PyVariable>() {
                        env = Some(rhs.env.clone());
                    } else if let Ok(rhs) = obj.extract::<PyExpression>() {
                        env = Some(match rhs.0 {
                            Left(expr) => expr.env.clone(),
                            Right(m) => m.access().environment.clone(),
                        })
                    }
                }
                match env {
                    Some(env) => Ok(PyExpression::new(Expression::empty(env))),
                    None => {
                        return Err(StartCannotBeInferredError::new_err(
                            "start cannot be inferred and no active environment found.",
                        ))
                    }
                }
            }
        }
    };
    let mut acc = start?;

    for item in items.into_iter() {
        let obj = item?;
        acc.__iadd__(py, obj.into())?;
    }

    Ok(acc)
}

use lunamodel_core::{ArcEnv, Expression};
use lunamodel_error::py::PyStartCannotBeInferredError;
use pyo3::{
    Bound, PyAny, PyResult, Python, pyfunction,
    types::{PyAnyMethods, PyTypeMethods},
};

use crate::{
    PyEnvironment, PyExpression, PyVariable, args::PyExprArg, environment::ACTIVE_ENV,
    utils::OpsOther,
};

#[pyfunction]
#[pyo3(signature=(iterable, start=None))]
pub fn quicksum(
    py: Python,
    iterable: &Bound<PyAny>,
    start: Option<PyExprArg>,
) -> PyResult<PyExpression> {
    let typestr = iterable.get_type().name()?.to_string().to_uppercase();
    let items: Vec<_> = if typestr.contains("ARRAY") {
        iterable.call_method0("flatten")?.try_iter()?.collect()
    } else {
        iterable.try_iter()?.collect()
    };
    // let items: Vec<_> = iterable.try_iter()?.collect();

    let start: PyResult<PyExpression> = if let Some(s) = start {
        Ok(s.into())
    } else {
        let env: Option<PyEnvironment> = ACTIVE_ENV.with(|current| current.borrow().clone());
        match env {
            Some(env) => Ok(Expression::empty(env.env).into()),
            None => {
                let mut env: Option<ArcEnv> = None;
                for item in items.iter() {
                    let obj = match item {
                        Ok(e) => e,
                        Err(err) => return Err(err.clone_ref(py)),
                    };
                    if let Ok(rhs) = obj.extract::<PyVariable>() {
                        env = Some(rhs.v.env.clone());
                    } else if let Ok(rhs) = obj.extract::<PyExpression>() {
                        env = Some(rhs.environment()?.env);
                    } else {
                        if let Ok(e) = obj.getattr("_expr") {
                            let expr: PyExpression = e.extract()?;
                            env = Some(expr.environment()?.env);
                        } else if let Ok(v) = obj.getattr("_v") {
                            let v: PyVariable = v.extract()?;
                            env = Some(v.v.env.clone());
                        }
                    }
                }
                match env {
                    Some(env) => Ok(PyExpression::new(Expression::empty(env))),
                    None => {
                        return Err(PyStartCannotBeInferredError::new_err(
                            "start cannot be inferred and no active environment found.",
                        ));
                    }
                }
            }
        }
    };
    let mut acc = start?;

    for item in items.into_iter() {
        let item = item?;
        if let Ok(e) = item.getattr("_expr") {
            acc.__iadd__(OpsOther::Expr(e.extract()?))?;
        } else if let Ok(v) = item.getattr("_v") {
            acc.__iadd__(OpsOther::Var(v.extract()?))?;
        } else {
            let obj = item.extract::<OpsOther>()?;
            acc.__iadd__(obj)?;
        }
        // TODO: return a well error in the else else case.
        // i.e., no Expression, PyExpression, Variable, PyVariable, or number
    }

    Ok(acc)
}

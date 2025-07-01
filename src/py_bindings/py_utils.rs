use either::Either::{Left, Right};
use pyo3::{
    prelude::{pyfunction, Bound, PyAny},
    types::PyAnyMethods,
    PyResult, Python,
};

use crate::core::{expression::ExpressionBaseCreation, Expression, SharedEnvironment};

use super::{
    py_env::{PyEnvironment, CURRENT_ENV},
    py_exceptions::StartCannotBeInferredError,
    py_expr::PyExpression,
    py_var::PyVariable,
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
                            Right(m) => m.borrow().environment.clone(),
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

    // for item in iterable.try_iter()? {
    //     let obj = item?;

    //     match acc.as_mut() {
    //         Some(acc) => acc.__iadd__(py, obj.into())?,
    //         None => {
    //             // Let's see if we can infer the type.
    //             if let Ok(rhs) = obj.extract::<PyVariable>() {
    //                 acc = Some(PyExpression::new(Expression::empty(rhs.env.clone())));
    //                 acc.as_mut().unwrap().__iadd__(py, obj.into())?
    //             } else if let Ok(rhs) = obj.extract::<PyExpression>() {
    //                 let env = match rhs.0 {
    //                     Left(expr) => expr.env.clone(),
    //                     Right(m) => m.borrow().environment.clone(),
    //                 };
    //                 acc = Some(PyExpression::new(Expression::empty(env)));
    //                 acc.as_mut().unwrap().__iadd__(py, obj.into())?
    //             } else {
    //             }
    //         }
    //     }
    // }
    // match acc {
    //     Some(res) => Ok(res),
    //     None => Err(NoActiveEnvironmentFoundError::new_err(
    //         "no active environment found.",
    //     )),
    // }
}

// #[pyfunction]
// #[pyo3(signature=(iterable, start=None))]
// pub fn quicksum(
//     py: Python,
//     iterable: &Bound<PyAny>,
//     start: Option<PyExpression>,
// ) -> PyResult<PyExpression> {
//     let mut acc: Option<PyExpression> = if let Some(s) = start {
//         Some(s)
//     } else {
//         let env: Option<PyEnvironment> = CURRENT_ENV.with(|current| {
//             current.borrow().clone()
//         });
//         match env {
//             Some(env) => Some(PyExpression::new(Expression::empty(env.0))),
//             None => None,
//         }
//     };
//
//
//
//     for item in iterable.try_iter()? {
//         let obj = item?;
//
//         match acc.as_mut() {
//             Some(acc) => acc.__iadd__(py, obj.into())?,
//             None => {
//                 // Let's see if we can infer the type.
//                 if let Ok(rhs) = obj.extract::<PyVariable>() {
//                     acc = Some(PyExpression::new(Expression::empty(rhs.env.clone())));
//                     acc.as_mut().unwrap().__iadd__(py, obj.into())?
//                 } else if let Ok(rhs) = obj.extract::<PyExpression>() {
//                     let env = match rhs.0 {
//                         Left(expr) => expr.env.clone(),
//                         Right(m) => m.borrow().environment.clone(),
//                     };
//                     acc = Some(PyExpression::new(Expression::empty(env)));
//                     acc.as_mut().unwrap().__iadd__(py, obj.into())?
//                 } else {
//
//                 }
//             }
//         }
//     }
//     match acc {
//         Some(res) => Ok(res),
//         None => Err(NoActiveEnvironmentFoundError::new_err(
//             "no active environment found.",
//         )),
//     }
// }

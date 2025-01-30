#[cfg(feature = "py")]
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "py")]
use pyo3::prelude::*;

use super::{
    environment::Environment,
    exceptions::VariableExistsException,
    expression::{CreateExpressionByOperation, Expression},
    term::{linear::Linear, number::Number, quadratic::Quadratic},
};

pub type VarId = u32;
pub static DEFAULT_SCALER_VALUE: f64 = 1.0;

#[cfg_attr(feature = "py", pyclass(name = "Variable", subclass))]
#[derive(Clone)]
pub struct VarRef {
    pub id: VarId,
}

impl VarRef {
    pub fn new(id: VarId) -> Self {
        Self { id }
    }
}

impl CreateExpressionByOperation<VarRef> for VarRef {
    fn add(&self, rhs: &VarRef) -> Expression {
        Expression::from_linear(Linear::from_vars(self, (rhs, None)))
    }
    fn sub(&self, rhs: &VarRef) -> Expression {
        // self - rhs
        Expression::from_linear(Linear::from_vars(self, (rhs, Some(-1.0))))
    }
    fn mul(&self, rhs: &VarRef) -> Expression {
        match self.id == rhs.id {
            true => {
                unimplemented!();
            }
            false => {
                return Expression::from_quadratic(Quadratic::from_vars(self, rhs));
            }
        }
    }
}

impl CreateExpressionByOperation<f64> for VarRef {
    fn add(&self, rhs: &f64) -> Expression {
        // rhs + self = self + rhs
        Expression::from_linear_and_constant(Linear::new(self, None), Number::new(*rhs))
    }
    fn sub(&self, rhs: &f64) -> Expression {
        // self - rhs
        Expression::from_linear_and_constant(Linear::new(self, None), Number::new(*rhs))
    }
    fn mul(&self, rhs: &f64) -> Expression {
        Expression::from_linear(Linear::new(self, Some(*rhs)))
    }
}

impl VarRef {
    fn rsub(&self, rhs: &f64) -> Expression {
        // rhs - self
        Expression::from_linear_and_constant(Linear::new(self, Some(-1.0)), Number::new(*rhs))
    }
}

//impl CreateExpressionByOperation<VarRef> for VarRef {
//    fn add(&self, rhs: &VarRef) -> Expression {
//        let mut expr = Expression::empty();
//        expr.add_assign(self);
//        expr.add_assign(rhs);
//        expr
//    }
//    fn sub(&self, rhs: &VarRef) -> Expression {
//        let mut expr = Expression::empty();
//        expr.sub_assign(self);
//        expr.sub_assign(rhs);
//        expr
//    }
//    /// Two variables are multiplied producing a new expression.
//    /// This is highly dependent on the type of the variable.
//    /// If both variables are binary, and are the same variable.
//    /// Then it produces a linear expression.
//    /// If they are the same then they produce a quadratic
//    /// expression
//    fn mul(&self, rhs: &VarRef) -> Expression {
//        match self.id == rhs.id {
//            true => {
//                unimplemented!();
//            }
//            false => {
//                return Expression::from_quadratic(Quadratic::new(self, rhs));
//            }
//        }
//    }
//}
//
//impl CreateExpressionByOperation<f64> for VarRef {
//    fn add(&self, rhs: &f64) -> Expression {
//        let mut expr = Expression::empty();
//        expr.add_assign(self);
//        expr.add_assign(rhs);
//        expr
//    }
//    fn sub(&self, rhs: &f64) -> Expression {
//        let mut expr = Expression::empty();
//        expr.sub_assign(self);
//        expr.sub_assign(rhs);
//        expr
//    }
//    fn mul(&self, rhs: &f64) -> Expression {
//        let mut expr = Expression::empty();
//        expr.add_assign(self);
//        expr.mul_assign(rhs);
//        expr
//    }
//}
//
//// // Adding two variables creates an Expression with only the linear term populated.
//// impl VarRef {
////     fn add(&self, rhs: &VarRef) -> Expression {
////         let mut expr = Expression::empty();
////         expr.linear.add_assign(self);
////         expr.linear.add_assign(rhs);
////         expr
////     }
//// }
//
//// impl Add<f64> for &VarRef {
////     type Output = Expression;
////
////     fn add(self, rhs: f64) -> Self::Output {
////         Expression::new_with_constant(self.id, rhs)
////     }
//// }
//
//// impl Add<VarRef> for &VarRef {
////     type Output = Expression;
////
////     fn add(self, rhs: VarRef) -> Self::Output {
////         let mut expr = Expression::empty();
////         expr.linear += self;
////         expr.linear += rhs;
////         expr
////     }
//// }
//
//// impl Add<&VarRef> for &VarRef {
////     type Output = Expression;
////
////     fn add(self, rhs: &VarRef) -> Self::Output {
////         let mut expr = Expression::empty();
////         expr.linear += self;
////         expr.linear += rhs;
////         expr
////     }
//// }
//
//// impl Mul<f64> for &VarRef {
////     type Output = Expression;
////
////     fn mul(self, rhs: f64) -> Self::Output {
////         Expression::new_with_linear(self.id, rhs)
////     }
//// }
////
//// impl Sub<f64> for &VarRef {
////     type Output = Expression;
////
////     fn sub(self, rhs: f64) -> Self::Output {
////         Expression::new_with_constant(self.id, -rhs)
////     }
//// }
///
///
// fn overloaded<F, T>(py: Python, other: PyObject, f: F) -> PyResult<Expression>
// where
//     F: Fn(&T) -> Expression,
// {
//     if let Ok(value) = &other.extract::<VarRef>(py) {
//         Ok(f(value))
//         // Ok(self.add(value))
//     } else if let Ok(value) = &other.extract::<f64>(py) {
//         Ok(f(value))
//         // Ok(self.add(value))
//     } else {
//         Err(PyRuntimeError::new_err("other type not recognized"))
//     }
// }

#[cfg(feature = "py")]
#[pymethods]
impl VarRef {
    #[new]
    #[pyo3(signature=(name, environment))]
    fn py_new(name: String, environment: &mut Environment) -> PyResult<VarRef> {
        environment.add_var(&name).map_err(|_| {
            VariableExistsException::new_err(format!(
                "variable with name '{}' already exists",
                name
            ))
        })
    }

    fn name(&self, environment: &Environment) -> String {
        environment.get_var(self.id).name.clone()
    }

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(value) = &other.extract::<VarRef>(py) {
            Ok(self.add(value))
        } else if let Ok(value) = &other.extract::<f64>(py) {
            Ok(self.add(value))
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __radd__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        self.__add__(py, other)
    }

    fn __mul__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(value) = &other.extract::<VarRef>(py) {
            Ok(self.mul(value))
        } else if let Ok(value) = &other.extract::<f64>(py) {
            Ok(self.mul(value))
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __rmul__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        self.__mul__(py, other)
    }

    fn __sub__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(value) = &other.extract::<VarRef>(py) {
            Ok(self.sub(value))
        } else if let Ok(value) = &other.extract::<f64>(py) {
            Ok(self.sub(value))
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __rsub__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(value) = &other.extract::<f64>(py) {
            Ok(self.rsub(value))
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }
}

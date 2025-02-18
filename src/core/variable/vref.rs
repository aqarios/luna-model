// use std::ops::{Add, Mul};

use std::{cell::RefCell, fmt::Display, rc::Rc};

// #[cfg(feature = "py")]
// use pyo3::exceptions::PyRuntimeError;
// #[cfg(feature = "py")]
// use pyo3::prelude::*;

use crate::core::{
    Environment,
    // exceptions::VariablesFromDifferentEnvsError,
    // expression::Expression,
    // term::{Constant, Linear, Quadratic},
};

// #[cfg(feature = "py")]
// use crate::core::exceptions::VariableExistsException;
// use crate::core::exceptions::{VariableExistsException, VariablesFromDifferentEnvsException};

// #[cfg(feature = "py")]
// use crate::core::Environment;

// #[cfg(feature = "py")]
// use super::{Bounds, Vtype};

#[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct VarId(pub u32);

impl Into<usize> for VarId {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for VarId {
    fn from(value: usize) -> Self {
        assert!(value <= u32::MAX as usize, "value out of range for u32");
        VarId(value as u32)
    }
}

impl Into<u64> for VarId {
    fn into(self) -> u64 {
        self.0 as u64
    }
}

impl Display for VarId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// #[cfg_attr(feature = "py", pyclass(name = "Variable", subclass))]
// #[derive(Clone)]
pub struct VarRef {
    pub id: VarId,
    pub env: Rc<RefCell<Environment>>,
    // pub env_id: EnvId,
}

impl VarRef {
    pub fn new(id: u32, env: Rc<RefCell<Environment>>) -> Self {
        Self { id: VarId(id), env }
    }

    pub fn new_from_id(id: VarId, env: Rc<RefCell<Environment>>) -> Self {
        Self { id, env }
    }

    pub fn key(&self) -> u64 {
        self.id.into()
    }
}

impl Drop for VarRef {
    fn drop(&mut self) {
        self.env.borrow_mut().drop_var(self.id)
    }
}

// impl Add<&VarRef> for &VarRef {
//     type Output = Result<Expression, VariablesFromDifferentEnvsError>;
//
//     fn add(self, rhs: &VarRef) -> Self::Output {
//         let linear = Linear::new_from_vars((self, 1.0), (rhs, 1.0))?;
//         Ok(Expression::new_from_linear(linear))
//     }
// }
//
// impl Add<&f64> for &VarRef {
//     type Output = Expression;
//
//     fn add(self, rhs: &f64) -> Self::Output {
//         Expression::new_from_linear_with_constant(Linear::new((self, 1.0)), Constant::new(*rhs))
//     }
// }
//
// impl Mul<&f64> for &VarRef {
//     type Output = Expression;
//
//     fn mul(self, rhs: &f64) -> Self::Output {
//         Expression::new_from_linear(Linear::new((self, *rhs)))
//     }
// }
//
// impl Mul<&f64> for VarRef {
//     type Output = Expression;
//
//     fn mul(self, rhs: &f64) -> Self::Output {
//         Expression::new_from_linear(Linear::new((&self, *rhs)))
//     }
// }
//
// impl Mul<&VarRef> for &VarRef {
//     type Output = Result<Expression, VariablesFromDifferentEnvsError>;
//
//     fn mul(self, rhs: &VarRef) -> Self::Output {
//         let quadratic = Quadratic::new_from_vars(self, rhs)?;
//         Ok(Expression::new_from_quadratic(quadratic))
//     }
// }

// impl Mul<VarRef> for VarRef {
//     type Output = Expression;
//
//     fn mul(self, rhs: VarRef) -> Self::Output {
//         // If the same variable is used and it is binary we have a linear.
//         // If the same variable and we have spin we have constant.
//         // else we have quadratic
//         let quadratic = Quadratic::new_from_vars(self, rhs)?;
//         Expression::new_from_quadratic(quadratic)
//     }
// }

// #[cfg(feature = "py")]
// #[pymethods]
// impl VarRef {
//     #[new]
//     #[pyo3(signature=(name, environment, vtype=None, bounds=None))]
//     fn py_new(
//         name: String,
//         environment: &mut Environment,
//         vtype: Option<Vtype>,
//         bounds: Option<Bounds>,
//     ) -> PyResult<VarRef> {
//         environment
//             .add_var(&name, vtype, bounds)
//             .map_err(|e| VariableExistsException::new_err(format!("{}: {}", e.to_string(), name)))
//     }
//
//     // fn __add__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
//     //     if let Ok(v) = &other.extract::<VarRef>(py) {
//     //         self.add(v)
//     //             .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
//     //     } else if let Ok(v) = &other.extract::<f64>(py) {
//     //         Ok(self.add(v))
//     //     } else {
//     //         Err(PyRuntimeError::new_err("unsopported type for operation"))
//     //     }
//     // }
//
//     // fn __radd__(&self, other: f64) -> Expression {
//     //     self.add(&other)
//     // }
//
//     // fn __mul__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
//     //     if let Ok(v) = &other.extract::<f64>(py) {
//     //         Ok(self.mul(v))
//     //     } else if let Ok(v) = &other.extract::<VarRef>(py) {
//     //         self.mul(v)
//     //             .map_err(|e| VariablesFromDifferentEnvsException::new_err(e.to_string()))
//     //     } else {
//     //         Err(PyRuntimeError::new_err("unsopported type for operation"))
//     //     }
//     // }
//
//     // fn __rmul__(&self, other: f64) -> Expression {
//     //     self.mul(&other)
//     // }
// }

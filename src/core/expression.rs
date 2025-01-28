use std::{
    fmt::Display,
    ops::{Add, AddAssign, MulAssign, Sub, SubAssign},
};

use super::{
    term::{higher_order::HigherOrder, linear::Linear, number::Number, quadratic::Quadratic},
    varref::{VarId, VarRef},
};

#[cfg(feature = "py")]
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "py")]
use pyo3::prelude::*;

#[cfg_attr(feature = "py", pyclass(subclass))]
#[derive(Clone, PartialEq)]
pub struct Expression {
    pub linear: Linear,
    pub quadratic: Quadratic,
    pub higher_order: HigherOrder,
    pub constant: Number,
}

impl Expression {
    pub fn new_with_constant(varid: VarId, constant: f64) -> Self {
        Expression {
            linear: Linear::new(varid, None),
            quadratic: Quadratic::empty(),
            higher_order: HigherOrder::empty(),
            constant: Number::new(constant),
        }
    }
    pub fn new_with_linear(varid: VarId, value: f64) -> Self {
        Expression {
            linear: Linear::new(varid, Some(value)),
            quadratic: Quadratic::empty(),
            higher_order: HigherOrder::empty(),
            constant: Number::empty(),
        }
    }

    pub fn empty() -> Self {
        Self {
            linear: Linear::empty(),
            quadratic: Quadratic::empty(),
            higher_order: HigherOrder::empty(),
            constant: Number::empty(),
        }
    }

    pub fn as_string(&self) -> String {
        if self.constant >= 0.0 {
            format!("{} + {}", self.linear, self.constant)
        } else {
            format!("{} - {}", self.linear, self.constant.value * -1.0)
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "self.as_string() = {}", self.as_string())
    }
}

// impl AddAssign<Variable> for Expression {
//     fn add_assign(&mut self, rhs: Variable) {
//         // self.linear += rhs
//         self.linear.add_var(&rhs);
//     }
// }
//
// impl AddAssign<&Variable> for Expression {
//     fn add_assign(&mut self, rhs: &Variable) {
//         //self.linear += rhs
//         self.linear.add_var(rhs);
//     }
// }

impl Add<f64> for Expression {
    type Output = Expression;
    fn add(self, rhs: f64) -> Self::Output {
        let mut expr = Expression::empty();
        expr.linear = self.linear;
        expr.constant = self.constant + rhs;
        expr
    }
}

impl Add<f64> for &Expression {
    type Output = Expression;
    fn add(self, rhs: f64) -> Self::Output {
        let mut expr = Expression::empty();
        expr.linear.variables = self.linear.variables.clone();
        expr.constant = Number::new(self.constant.value + rhs);
        expr
    }
}

impl Add<&Expression> for &Expression {
    type Output = Expression;
    fn add(self, rhs: &Expression) -> Self::Output {
        let mut expr = Expression::empty();
        expr.constant = &self.constant + &rhs.constant;
        expr.linear = &self.linear + &rhs.linear;
        // todo: quadratic, higher order
        // expr.quadratic = self.quadratic + rhs.quadratic;
        // expr.higher_order = self.higher_order + rhs.higher_order;
        expr
    }
}

impl Sub<f64> for Expression {
    type Output = Expression;
    fn sub(self, rhs: f64) -> Self::Output {
        let mut expr = Expression::empty();
        expr.linear.variables = self.linear.variables.clone();
        expr.constant = Number::new(self.constant.value - rhs);
        expr
    }
}

impl Sub<f64> for &Expression {
    type Output = Expression;
    fn sub(self, rhs: f64) -> Self::Output {
        let mut expr = Expression::empty();
        expr.linear.variables = self.linear.variables.clone();
        expr.constant = Number::new(self.constant.value - rhs);
        expr
    }
}

impl Sub<&Expression> for &Expression {
    type Output = Expression;
    fn sub(self, rhs: &Expression) -> Self::Output {
        let mut expr = Expression::empty();
        expr.constant = &self.constant - &rhs.constant;
        expr.linear = &self.linear - &rhs.linear;
        // todo: quadratic, higher order
        // expr.quadratic = self.quadratic + rhs.quadratic;
        // expr.higher_order = self.higher_order + rhs.higher_order;
        expr
    }
}

impl Add<&VarRef> for &Expression {
    type Output = Expression;

    fn add(self, rhs: &VarRef) -> Self::Output {
        let mut expr = Expression::empty();
        expr.constant = self.constant;
        expr.linear = self.linear.clone();
        expr.linear += rhs;
        expr
    }
}

impl Sub<&VarRef> for &Expression {
    type Output = Expression;
    fn sub(self, rhs: &VarRef) -> Self::Output {
        let mut expr = Expression::empty();
        expr.constant = self.constant;
        expr.linear = self.linear.clone();
        expr.linear -= rhs;
        // todo: quadratic, higher order
        // expr.quadratic = self.quadratic + rhs.quadratic;
        // expr.higher_order = self.higher_order + rhs.higher_order;
        expr
    }
}

impl AddAssign<f64> for Expression {
    fn add_assign(&mut self, rhs: f64) {
        self.constant += rhs
    }
}

impl AddAssign<Expression> for Expression {
    fn add_assign(&mut self, rhs: Expression) {
        self.constant += rhs.constant;
        self.linear += rhs.linear;
        // todo: quadratic, higher order
        // self.quadratic += rhs.quadratic;
        // self.higher_order += rhs.higher_order;
    }
}

impl AddAssign<VarRef> for Expression {
    fn add_assign(&mut self, rhs: VarRef) {
        self.linear += rhs
    }
}

impl SubAssign<f64> for Expression {
    fn sub_assign(&mut self, rhs: f64) {
        self.constant -= rhs
    }
}

impl SubAssign<VarRef> for Expression {
    fn sub_assign(&mut self, rhs: VarRef) {
        self.linear -= rhs
    }
}

impl SubAssign<&VarRef> for Expression {
    fn sub_assign(&mut self, rhs: &VarRef) {
        self.linear -= rhs
    }
}

impl SubAssign<Expression> for Expression {
    fn sub_assign(&mut self, rhs: Expression) {
        self.linear -= rhs.linear;
        self.constant -= rhs.constant;
        // todo: rest
    }
}

// impl SubAssign<Expression> for Expression {
//     fn sub_assign(&mut self, rhs: Expression) {
//         self.linear -= rhs.linear;
//         self.constant -= rhs.constant;
//         // todo: rest
//     }
// }

impl MulAssign<f64> for Expression {
    fn mul_assign(&mut self, rhs: f64) {
        self.constant *= rhs
    }
}

#[cfg(feature = "py")]
#[pymethods]
impl Expression {
    #[new]
    fn py_new() -> Self {
        Self::empty()
    }

    fn __str__(&self) -> String {
        self.as_string()
    }

    fn __repr__(&self) -> String {
        self.as_string()
    }

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(value) = &other.extract::<VarRef>(py) {
            let expr = self + value;
            Ok(expr)
        } else if let Ok(value) = &other.extract::<Expression>(py) {
            let expr = self + value;
            Ok(expr)
        } else if let Ok(value) = other.extract::<f64>(py) {
            let expr = self + value;
            Ok(expr)
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(value) = other.extract::<VarRef>(py) {
            *self += value;
            Ok(())
        } else if let Ok(value) = other.extract::<Expression>(py) {
            *self += value;
            Ok(())
        } else if let Ok(value) = other.extract::<f64>(py) {
            *self += value;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __sub__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(value) = &other.extract::<VarRef>(py) {
            let expr = self - value;
            Ok(expr)
        } else if let Ok(value) = &other.extract::<Expression>(py) {
            let expr = self - value;
            Ok(expr)
        } else if let Ok(value) = other.extract::<f64>(py) {
            let expr = self - value;
            Ok(expr)
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __isub__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(value) = other.extract::<VarRef>(py) {
            *self -= value;
            Ok(())
        } else if let Ok(value) = other.extract::<Expression>(py) {
            *self -= value;
            Ok(())
        } else if let Ok(value) = other.extract::<f64>(py) {
            *self -= value;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __eq__(&self, other: &Expression) -> bool {
        self == other
    }

    fn __req__(&self, other: &Expression) -> bool {
        self == other
    }
}

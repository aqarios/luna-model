use std::fmt::Display;

use super::term::{
    higher_order::HigherOrder, linear::Linear, number::Number, quadratic::Quadratic, TermAddition,
    TermMultiplication, TermSubtraction,
};
use super::variable::VarRef;

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
    pub fn from_quadratic(quadratic: Quadratic) -> Self {
        Expression {
            linear: Linear::empty(),
            quadratic,
            higher_order: HigherOrder::empty(),
            constant: Number::empty(),
        }
    }

    pub fn from_linear(linear: Linear) -> Self {
        Expression {
            linear,
            quadratic: Quadratic::empty(),
            higher_order: HigherOrder::empty(),
            constant: Number::empty(),
        }
    }

    pub fn from_constant(constant: Number) -> Self {
        Expression {
            linear: Linear::empty(),
            quadratic: Quadratic::empty(),
            higher_order: HigherOrder::empty(),
            constant,
        }
    }

    pub fn from_linear_and_constant(linear: Linear, constant: Number) -> Self {
        Expression {
            linear,
            quadratic: Quadratic::empty(),
            higher_order: HigherOrder::empty(),
            constant,
        }
    }
}

// maybe move into addition and subtraction and let this function check?
// might have a performance implact.
// It might be necessary to the stuff out and do on a higher level.
// might enable faster processing...
pub trait CreateExpressionByOperation<T> {
    fn add(&self, rhs: &T) -> Expression;
    fn mul(&self, rhs: &T) -> Expression;
    fn sub(&self, rhs: &T) -> Expression;
    // fn rsub(&self, rhs: &T) -> Expression;
    // fn add_assign(&self, lhs: &A, rhs: &B) -> Expression;
}

pub trait Creation {
    fn from_other(base: &Self) -> Self;
}

impl Creation for Expression {
    fn from_other(other: &Self) -> Self {
        Self {
            constant: other.constant,
            linear: other.linear.clone(),
            quadratic: other.quadratic.clone(),
            higher_order: other.higher_order.clone(),
        }
    }
}

pub trait Addition<T> {
    fn add(&self, rhs: &T) -> Self
    where
        Self: Sized + Creation,
    {
        let mut out = Creation::from_other(self);
        Addition::add_assign(&mut out, rhs);
        out
    }

    fn add_assign(&mut self, rhs: &T);
}

pub trait Subtraction<T> {
    fn sub(&self, rhs: &T) -> Self
    where
        Self: Sized + Creation,
    {
        let mut out = Creation::from_other(self);
        Subtraction::sub_assign(&mut out, rhs);
        out
    }

    fn sub_assign(&mut self, rhs: &T);
}

pub trait Multiplication<T> {
    fn mul(&self, rhs: &T) -> Self
    where
        Self: Sized + Creation,
    {
        let mut out = Creation::from_other(self);
        Multiplication::mul_assign(&mut out, rhs);
        out
    }

    fn mul_assign(&mut self, rhs: &T);
}

impl Addition<VarRef> for Expression {
    fn add_assign(&mut self, var: &VarRef) {
        self.linear.add_assign(var);
    }
}

impl Addition<f64> for Expression {
    fn add_assign(&mut self, rhs: &f64) {
        self.constant.add_assign(rhs);
    }
}

impl Addition<Expression> for Expression {
    fn add_assign(&mut self, rhs: &Expression) {
        self.constant.add_assign(&rhs.constant);
        self.linear.add_assign(&rhs.linear);
        self.quadratic.add_assign(&rhs.quadratic);
        self.higher_order.add_assign(&rhs.higher_order)
    }
}

impl Subtraction<VarRef> for Expression {
    fn sub_assign(&mut self, var: &VarRef) {
        self.linear.sub_assign(var);
    }
}

impl Subtraction<f64> for Expression {
    fn sub_assign(&mut self, rhs: &f64) {
        self.constant.sub_assign(rhs);
    }
}

impl Subtraction<Expression> for Expression {
    fn sub_assign(&mut self, rhs: &Expression) {
        self.constant.sub_assign(&rhs.constant);
        self.linear.sub_assign(&rhs.linear);
        self.quadratic.sub_assign(&rhs.quadratic);
        self.higher_order.sub_assign(&rhs.higher_order)
    }
}

impl Multiplication<f64> for Expression {
    fn mul_assign(&mut self, rhs: &f64) {
        self.constant.mul_assign(rhs);
        self.linear.mul_assign(rhs);
        self.quadratic.mul_assign(rhs);
        self.higher_order.mul_assign(rhs);
    }
}

impl Multiplication<VarRef> for Expression {
    fn mul_assign(&mut self, rhs: &f64) {
        unimplemented!()
    }
}

// fn add_const(&self, constant: &f64) -> Self {
//     let mut new = Self::from(self);
//     new.add_const_assign(constant);
//     new
// }

// fn add_expr(&self, expr: &Expression) -> Self {
//     unimplemented!()
// }

// impl Expression {
//     // pub fn add_var_assign(&mut self, var: &VarRef) {}
//
//     pub fn add_const_assign(&mut self, constant: &f64) {
//         self.constant.add_assign(constant);
//     }
//
//     fn add_expr_assign(&mut self, expr: &Expression) {
//         self.constant.add_assign(&expr.constant);
//     }
// }

// impl Expression {
//     fn sub_var(&self, var: &VarRef) -> Self {
//         // this might be optimizable
//         let mut new = Self::from_other(self);
//         new.sub_var_assign(var);
//         new
//     }
//     fn sub_const(&self, var: &VarRef) -> Self {
//         // this might be optimizable
//         let mut new = Self::from_other(self);
//         new.sub_var_assign(var);
//         new
//     }
// }
//
// impl Expression {
//     pub fn sub_var_assign(&mut self, var: &VarRef) {
//         self.linear.sub_assign(var);
//     }
//
//     // todo
//     // pub fn mul_var_assign(&mut self, var: &VarRef) {
//     //     let new_quadratic = self.linear.mul_var(var);
//
//     //     // self.quadratic.mul_var(var);
//     // }
// }

impl Expression {
    // pub fn new_with_constant(varid: VarId, constant: f64) -> Self {
    //     Expression {
    //         linear: Linear::new(varid, None),
    //         quadratic: Quadratic::empty(),
    //         higher_order: HigherOrder::empty(),
    //         constant: Number::new(constant),
    //     }
    // }
    // pub fn new_with_linear(varid: VarId, value: f64) -> Self {
    //     Expression {
    //         linear: Linear::new(varid, Some(value)),
    //         quadratic: Quadratic::empty(),
    //         higher_order: HigherOrder::empty(),
    //         constant: Number::empty(),
    //     }
    // }

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

// impl Add<f64> for Expression {
//     type Output = Expression;
//     fn add(self, rhs: f64) -> Self::Output {
//         let mut expr = Expression::empty();
//         expr.linear = self.linear;
//         expr.constant = self.constant + rhs;
//         expr
//     }
// }
//
// impl Add<f64> for &Expression {
//     type Output = Expression;
//     fn add(self, rhs: f64) -> Self::Output {
//         let mut expr = Expression::empty();
//         expr.linear.variables = self.linear.variables.clone();
//         expr.constant = Number::new(self.constant.value + rhs);
//         expr
//     }
// }
//
// impl Add<&Expression> for &Expression {
//     type Output = Expression;
//     fn add(self, rhs: &Expression) -> Self::Output {
//         let mut expr = Expression::empty();
//         expr.constant = &self.constant + &rhs.constant;
//         expr.linear = &self.linear + &rhs.linear;
//         // todo: quadratic, higher order
//         // expr.quadratic = self.quadratic + rhs.quadratic;
//         // expr.higher_order = self.higher_order + rhs.higher_order;
//         expr
//     }
// }
//
// impl Sub<f64> for Expression {
//     type Output = Expression;
//     fn sub(self, rhs: f64) -> Self::Output {
//         let mut expr = Expression::empty();
//         expr.linear.variables = self.linear.variables.clone();
//         expr.constant = Number::new(self.constant.value - rhs);
//         expr
//     }
// }
//
// impl Sub<f64> for &Expression {
//     type Output = Expression;
//     fn sub(self, rhs: f64) -> Self::Output {
//         let mut expr = Expression::empty();
//         expr.linear.variables = self.linear.variables.clone();
//         expr.constant = Number::new(self.constant.value - rhs);
//         expr
//     }
// }
//
// impl Sub<&Expression> for &Expression {
//     type Output = Expression;
//     fn sub(self, rhs: &Expression) -> Self::Output {
//         let mut expr = Expression::empty();
//         expr.constant = &self.constant - &rhs.constant;
//         expr.linear = &self.linear - &rhs.linear;
//         // todo: quadratic, higher order
//         // expr.quadratic = self.quadratic + rhs.quadratic;
//         // expr.higher_order = self.higher_order + rhs.higher_order;
//         expr
//     }
// }
//
// impl Add<&VarRef> for &Expression {
//     type Output = Expression;
//
//     fn add(self, rhs: &VarRef) -> Self::Output {
//         let mut expr = Expression::empty();
//         expr.constant = self.constant;
//         expr.linear = self.linear.clone();
//         expr.linear += rhs;
//         expr
//     }
// }
//
// impl Sub<&VarRef> for &Expression {
//     type Output = Expression;
//     fn sub(self, rhs: &VarRef) -> Self::Output {
//         let mut expr = Expression::empty();
//         expr.constant = self.constant;
//         expr.linear = self.linear.clone();
//         expr.linear -= rhs;
//         // todo: quadratic, higher order
//         // expr.quadratic = self.quadratic + rhs.quadratic;
//         // expr.higher_order = self.higher_order + rhs.higher_order;
//         expr
//     }
// }
//
// impl AddAssign<f64> for Expression {
//     fn add_assign(&mut self, rhs: f64) {
//         self.constant += rhs
//     }
// }
//
// impl AddAssign<Expression> for Expression {
//     fn add_assign(&mut self, rhs: Expression) {
//         self.constant += rhs.constant;
//         self.linear += rhs.linear;
//         // todo: quadratic, higher order
//         // self.quadratic += rhs.quadratic;
//         // self.higher_order += rhs.higher_order;
//     }
// }
//
// impl AddAssign<VarRef> for Expression {
//     fn add_assign(&mut self, rhs: VarRef) {
//         self.linear += rhs
//     }
// }
//
// impl SubAssign<f64> for Expression {
//     fn sub_assign(&mut self, rhs: f64) {
//         self.constant -= rhs
//     }
// }
//
// impl SubAssign<VarRef> for Expression {
//     fn sub_assign(&mut self, rhs: VarRef) {
//         self.linear -= rhs
//     }
// }
//
// impl SubAssign<&VarRef> for Expression {
//     fn sub_assign(&mut self, rhs: &VarRef) {
//         self.linear -= rhs
//     }
// }
//
// impl SubAssign<Expression> for Expression {
//     fn sub_assign(&mut self, rhs: Expression) {
//         self.linear -= rhs.linear;
//         self.constant -= rhs.constant;
//         // todo: rest
//     }
// }

// impl SubAssign<Expression> for Expression {
//     fn sub_assign(&mut self, rhs: Expression) {
//         self.linear -= rhs.linear;
//         self.constant -= rhs.constant;
//         // todo: rest
//     }
// }

// impl MulAssign<f64> for Expression {
//     fn mul_assign(&mut self, rhs: f64) {
//         self.constant *= rhs
//     }
// }
//

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
            Ok(self.add(value))
        } else if let Ok(value) = &other.extract::<Expression>(py) {
            Ok(self.add(value))
        } else if let Ok(value) = &other.extract::<f64>(py) {
            Ok(self.add(value))
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(value) = &other.extract::<VarRef>(py) {
            Ok(self.add_assign(value))
        } else if let Ok(value) = &other.extract::<Expression>(py) {
            Ok(self.add_assign(value))
        } else if let Ok(value) = &other.extract::<f64>(py) {
            Ok(self.add_assign(value))
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __sub__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(value) = &other.extract::<VarRef>(py) {
            Ok(self.sub(value))
        } else if let Ok(value) = &other.extract::<Expression>(py) {
            Ok(self.sub(value))
        } else if let Ok(value) = &other.extract::<f64>(py) {
            Ok(self.sub(value))
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __isub__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(value) = &other.extract::<VarRef>(py) {
            Ok(self.sub_assign(value))
        } else if let Ok(value) = &other.extract::<Expression>(py) {
            Ok(self.sub_assign(value))
        } else if let Ok(value) = &other.extract::<f64>(py) {
            Ok(self.sub_assign(value))
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    // Cannot use this as soon as we need the environment information...
    fn __mul__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        // if let Ok(value) = &other.extract::<VarRef>(py) {
        //     // Ok(self.mul(value))

        // } else if let Ok(value) = &other.extract::<Expression>(py) {
        //     // Ok(self.mul(value))
        // } else
        if let Ok(value) = &other.extract::<f64>(py) {
            Ok(self.mul(value))
        } else {
            Err(PyRuntimeError::new_err("other type not supported"))
        }
    }

    fn __eq__(&self, other: &Expression) -> bool {
        self == other
    }

    fn __req__(&self, other: &Expression) -> bool {
        self == other
    }
}

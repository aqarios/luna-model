use std::ops::{Add, AddAssign, Sub, SubAssign};

use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::{
    environment::EnvId,
    exceptions::{DifferentEnvsError, DifferentEnvsException},
    term::{Constant, HigherOrder, Linear, Quadratic},
};

#[cfg_attr(feature = "py", pyclass(subclass))]
#[derive(Clone, PartialEq)]
pub struct Expression {
    // optional to enhance the performance for lean moedls, i.e., only
    // linear models etc. as the other data does not needlessly be copied/cloned.
    pub env_id: EnvId,
    pub constant: Constant,
    pub linear: Linear,
    pub quadratic: Quadratic,
    pub higher_order: HigherOrder,
}

impl Expression {
    pub fn empty(env_id: EnvId) -> Self {
        Self {
            env_id,
            constant: Constant::empty(),
            linear: Linear::empty(env_id),
            quadratic: Quadratic::empty(env_id),
            higher_order: HigherOrder::empty(env_id),
        }
    }
    pub fn new(
        env_id: EnvId,
        constant: Constant,
        linear: Linear,
        quadratic: Quadratic,
        higher_order: HigherOrder,
    ) -> Result<Self, DifferentEnvsError> {
        Self::check_env_ids(&linear, &quadratic, &higher_order)?;
        Ok(Self {
            env_id,
            constant,
            linear,
            quadratic,
            higher_order,
            // constant: constant.unwrap_or(Constant::empty()),
            // linear: linear.unwrap_or(Linear::empty()),
            // quadratic: quadratic.unwrap_or(Quadratic::empty()),
            // higher_order: higher_order.unwrap_or(HigherOrder::empty()),
        })
    }

    pub fn new_from_linear_with_constant(linear: Linear, constant: Constant) -> Self {
        Self {
            env_id: linear.env_id,
            quadratic: Quadratic::empty(linear.env_id),
            higher_order: HigherOrder::empty(linear.env_id),
            constant,
            linear,
        }
    }

    pub fn new_from_linear(linear: Linear) -> Self {
        Self {
            env_id: linear.env_id,
            constant: Constant::empty(),
            quadratic: Quadratic::empty(linear.env_id),
            higher_order: HigherOrder::empty(linear.env_id),
            linear,
        }
    }

    pub fn new_from_quadratic(quadratic: Quadratic) -> Self {
        Self {
            env_id: quadratic.env_id,
            constant: Constant::empty(),
            linear: Linear::empty(quadratic.env_id),
            higher_order: HigherOrder::empty(quadratic.env_id),
            quadratic,
        }
    }

    pub fn new_from_higher_order(higher_order: HigherOrder) -> Self {
        Self {
            env_id: higher_order.env_id,
            constant: Constant::empty(),
            linear: Linear::empty(higher_order.env_id),
            quadratic: Quadratic::empty(higher_order.env_id),
            higher_order,
        }
    }

    fn check_env_ids(
        linear: &Linear,
        quadratic: &Quadratic,
        higher_order: &HigherOrder,
    ) -> Result<(), DifferentEnvsError> {
        if linear.env_id == quadratic.env_id && quadratic.env_id == higher_order.env_id {
            Ok(())
        } else {
            Err(DifferentEnvsError)
        }
        // let ok: bool;
        // match (linear, quadratic, higher_order) {
        //     (l, Some(q), None) => ok = l.env_id == q.env_id,
        //     (l, None, Some(h)) => ok = l.env_id == h.env_id,
        //     (l, Some(q), Some(h)) => ok = l.env_id == q.env_id && q.env_id == h.env_id,
        //     (_, _, _) => ok = true,
        // };

        // if ok {
        //     Ok(())
        // } else {
        //     Err(DifferentEnvsError)
        // }
    }

    fn check_env_id(&self, other: &Expression) -> Result<(), DifferentEnvsError> {
        if self.env_id != other.env_id {
            Err(DifferentEnvsError)
        } else {
            Ok(())
        }
    }
}

impl Add<&Expression> for &Expression {
    type Output = Result<Expression, DifferentEnvsError>;

    fn add(self, rhs: &Expression) -> Self::Output {
        self.check_env_id(rhs)?;
        Ok(Expression::new(
            self.env_id,
            self.constant.add(&rhs.constant),
            self.linear.add(&rhs.linear),
            self.quadratic.add(&rhs.quadratic),
            self.higher_order.add(&rhs.higher_order),
        )?)
    }
}

pub trait Addition<T> {
    fn add_assign(&mut self, rhs: T) -> Result<(), DifferentEnvsError>;
}

impl Addition<&Expression> for Expression {
    fn add_assign(&mut self, rhs: &Expression) -> Result<(), DifferentEnvsError> {
        self.check_env_id(rhs)?;
        self.constant.add_assign(&rhs.constant);
        self.linear.add_assign(&rhs.linear);
        self.quadratic.add_assign(&rhs.quadratic);
        self.higher_order.add_assign(&rhs.higher_order);
        Ok(())
    }
}

impl Sub<&Expression> for &Expression {
    type Output = Result<Expression, DifferentEnvsError>;

    fn sub(self, rhs: &Expression) -> Self::Output {
        self.check_env_id(rhs)?;
        Ok(Expression::new(
            self.env_id,
            self.constant.sub(&rhs.constant),
            self.linear.sub(&rhs.linear),
            self.quadratic.sub(&rhs.quadratic),
            self.higher_order.sub(&rhs.higher_order),
        )?)
    }
}

pub trait Subtraction<T> {
    fn sub_assign(&mut self, rhs: T) -> Result<(), DifferentEnvsError>;
}

impl Subtraction<&Expression> for Expression {
    fn sub_assign(&mut self, rhs: &Expression) -> Result<(), DifferentEnvsError> {
        self.check_env_id(rhs)?;
        self.constant.sub_assign(&rhs.constant);
        self.linear.sub_assign(&rhs.linear);
        self.quadratic.sub_assign(&rhs.quadratic);
        self.higher_order.sub_assign(&rhs.higher_order);
        Ok(())
    }
}

#[pymethods]
impl Expression {
    fn __add__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(v) = &other.extract::<Expression>(py) {
            self.add(v)
                .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(v) = &other.extract::<Expression>(py) {
            self.add_assign(v)
                .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn __sub__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(v) = &other.extract::<Expression>(py) {
            self.sub(v)
                .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn __isub__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(v) = &other.extract::<Expression>(py) {
            self.sub_assign(v)
                .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    // fn __mul__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
    //     if let Ok(v) = &other.extract::<f64>(py) {
    //         self.mul(v)
    //             .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
    //     } else {
    //         Err(PyRuntimeError::new_err("unsopported type for operation"))
    //     }
    // }
}

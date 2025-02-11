use crate::core::{
    expression::multiplications::{
        constant_times_constant, constant_times_higher_order, constant_times_term,
        higher_order_times_higher_order, higher_order_times_linear, linear_times_higher_order,
        linear_times_linear, linear_times_quadratic, quadratic_times_higher_order,
        quadratic_times_linear, quadratic_times_quadratic,
    },
    higher_order_operations::{
        TermAdditionC, TermC, TermConstantMultiplicationC, TermFloatMultiplicationC,
        TermMultiplication2, TermMultiplication3, TermMultiplicationC, TermSubtractionC,
        TermVarMultiplicationC,
    },
    operations::{
        Term, TermAddition, TermConstantMultiplication, TermFloatMultiplication,
        TermLinearMultiplication, TermMultiplication, TermSubtraction, TermVarMultiplication,
    },
    VarRef,
};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::{
    environment::EnvId,
    exceptions::{DifferentEnvsError, DifferentEnvsException},
    term::{Constant, HigherOrder, Linear, Quadratic},
    Environment,
};

use super::multiplications::higher_order_times_quadratic;

#[cfg_attr(feature = "py", pyclass(subclass))]
#[derive(Clone, PartialEq)]
pub struct Expression {
    //  (team) - 10.02.2025
    // A size could increase processing speeds for two expressions.
    // The larger expression is edited or cloned and the smaller expression
    // is iterated. This should decrease the required runtime. Furhter, thoughts
    // have to be invested tho.
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
        })
    }

    pub fn new_unchecked(
        env_id: EnvId,
        constant: Constant,
        linear: Linear,
        quadratic: Quadratic,
        higher_order: HigherOrder,
    ) -> Self {
        Self {
            env_id,
            constant,
            linear,
            quadratic,
            higher_order,
        }
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
    }

    fn check_env_id(&self, other: &Expression) -> Result<(), DifferentEnvsError> {
        if self.env_id != other.env_id {
            Err(DifferentEnvsError)
        } else {
            Ok(())
        }
    }

    fn check_env_id_var(&self, other: &VarRef) -> Result<(), DifferentEnvsError> {
        if self.env_id != other.env_id {
            Err(DifferentEnvsError)
        } else {
            Ok(())
        }
    }

    fn as_string(&self, environment: &Environment) -> String {
        // let mut strings = vec![
        //     self.higher_order.as_string(environment),
        //     self.quadratic.as_string(environment),
        //     self.linear.as_string(environment),
        //     self.constant.as_string(),
        // ];
        // strings.retain(|s| s.chars().count() != 0);
        // strings.join(" + ")
        format!(
            "constant: {}\nlinear: {}\nquadratic: {}\nhigher order: {}",
            self.constant.as_string(),
            self.linear.as_string(environment),
            self.quadratic.as_string(environment),
            self.higher_order.as_string(environment)
        )
    }
}

impl Add<f64> for &Expression {
    type Output = Expression;

    fn add(self, rhs: f64) -> Self::Output {
        Expression::new_unchecked(
            self.env_id,
            self.constant.add(rhs),
            self.linear.clone(),
            self.quadratic.clone(),
            self.higher_order.clone(),
        )
    }
}

impl Sub<f64> for &Expression {
    type Output = Expression;

    fn sub(self, rhs: f64) -> Self::Output {
        Expression::new_unchecked(
            self.env_id,
            self.constant.sub(rhs),
            self.linear.clone(),
            self.quadratic.clone(),
            self.higher_order.clone(),
        )
    }
}

impl Mul<f64> for &Expression {
    type Output = Expression;

    fn mul(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 {
            Expression::empty(self.env_id)
        } else {
            Expression::new_unchecked(
                self.env_id,
                self.constant.mul(rhs),
                TermFloatMultiplication::mul(&self.linear, rhs),
                TermFloatMultiplication::mul(&self.quadratic, rhs),
                TermFloatMultiplicationC::mul(&self.higher_order, rhs),
            )
        }
    }
}

impl Mul<(&VarRef, &Environment)> for &Expression {
    type Output = Result<Expression, DifferentEnvsError>;

    fn mul(self, rhs: (&VarRef, &Environment)) -> Self::Output {
        let (var, env) = rhs;
        self.check_env_id_var(var)?;
        // There is a new empty constant term, as the current constant is multiplied with
        // the passed variable. Thus, we can directly create a new empty constant term (0.0).
        let new_constant = Constant::empty();

        // Multiplying a variable to a linear term MIGHT result in a new
        // quadratic expression.
        let (mut new_linear, additional_quadratic) =
            TermVarMultiplication::mul(&self.linear, var, env);
        new_linear.append_variable(var, self.constant.value);

        let (mut new_quadratic, additional_higher_order) =
            TermVarMultiplicationC::mul(&self.quadratic, var, env);
        new_quadratic.append(additional_quadratic);

        let mut new_higher_order = TermMultiplicationC::mul(&self.higher_order, var, env);
        new_higher_order.append(additional_higher_order);

        Ok(Expression::new_unchecked(
            self.env_id,
            new_constant,
            new_linear,
            new_quadratic,
            new_higher_order,
        ))
    }
}

impl Mul<(&Expression, &Environment)> for &Expression {
    type Output = Result<Expression, DifferentEnvsError>;

    fn mul(self, rhs: (&Expression, &Environment)) -> Self::Output {
        let (other, env) = rhs;
        self.check_env_id(other)?;

        // self.constant x other.*
        let cc: Constant = constant_times_constant(&self.constant, &other.constant);
        let mut cl: Linear = constant_times_term(&self.constant, &other.linear);
        let mut cq: Quadratic = constant_times_term(&self.constant, &other.quadratic);
        let mut ch: HigherOrder = constant_times_higher_order(&self.constant, &other.higher_order);

        // // self.linear x other.*
        let lc: Linear = constant_times_term(&other.constant, &self.linear); // const x term = term x const
        let (ll, llq): (Linear, Option<Quadratic>) =
            linear_times_linear(&self.linear, &other.linear, env);

        let (lq, lqh): (Quadratic, Option<HigherOrder>) =
            linear_times_quadratic(&self.linear, &other.quadratic, env);
        let lh: HigherOrder = linear_times_higher_order(&self.linear, &other.higher_order, env);

        // self.quadratic x other.*
        let qc: Quadratic = constant_times_term(&other.constant, &self.quadratic); // con x term = term x con

        let (ql, qlh): (Quadratic, Option<HigherOrder>) =
            quadratic_times_linear(&self.quadratic, &other.linear, env);
        let (qq, qqh): (Quadratic, Option<HigherOrder>) =
            quadratic_times_quadratic(&self.quadratic, &other.quadratic, env);
        let qh: HigherOrder =
            quadratic_times_higher_order(&self.quadratic, &other.higher_order, env);

        // self.higher_order x other.*
        let hc: HigherOrder = constant_times_higher_order(&other.constant, &self.higher_order); // c x t = t x c
        let hl: HigherOrder = higher_order_times_linear(&self.higher_order, &other.linear, env);
        let hq: HigherOrder =
            higher_order_times_quadratic(&self.higher_order, &other.quadratic, env);
        let hh: HigherOrder =
            higher_order_times_higher_order(&self.higher_order, &other.higher_order, env);

        cl.append(Some(lc));
        cl.append(Some(ll));

        cq.append(llq);
        cq.append(Some(lq));
        cq.append(Some(qc));
        cq.append(Some(ql));
        cq.append(Some(qq));

        ch.append(lqh);
        ch.append(Some(lh));
        ch.append(qlh);
        ch.append(qqh);
        ch.append(Some(qh));
        ch.append(Some(hc));
        ch.append(Some(hl));
        ch.append(Some(hq));
        ch.append(Some(hh));

        // todo: combine all the results.
        Ok(Expression::new_unchecked(self.env_id, cc, cl, cq, ch))
    }
}

impl MulAssign<f64> for Expression {
    fn mul_assign(&mut self, rhs: f64) {
        if rhs == 0.0 {
            self.constant.reset();
            self.linear.reset();
            self.quadratic.reset();
            self.higher_order.reset();
        } else {
            self.constant.mul_assign(rhs);
            self.linear.mul_assign(rhs);
            self.quadratic.mul_assign(rhs);
            self.higher_order.mul_assign(rhs);
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

pub trait FailableAddAssign<T> {
    fn add_assign(&mut self, rhs: T) -> Result<(), DifferentEnvsError>;
}

impl FailableAddAssign<&Expression> for Expression {
    fn add_assign(&mut self, rhs: &Expression) -> Result<(), DifferentEnvsError> {
        self.check_env_id(rhs)?;
        self.constant.add_assign(&rhs.constant);
        self.linear.add_assign(&rhs.linear);
        self.quadratic.add_assign(&rhs.quadratic);
        self.higher_order.add_assign(&rhs.higher_order);
        Ok(())
    }
}

impl AddAssign<f64> for Expression {
    fn add_assign(&mut self, rhs: f64) {
        self.constant.add_assign(rhs);
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

pub trait FailableSubAssign<T> {
    fn sub_assign(&mut self, rhs: T) -> Result<(), DifferentEnvsError>;
}

impl FailableSubAssign<&Expression> for Expression {
    fn sub_assign(&mut self, rhs: &Expression) -> Result<(), DifferentEnvsError> {
        self.check_env_id(rhs)?;
        self.constant.sub_assign(&rhs.constant);
        self.linear.sub_assign(&rhs.linear);
        self.quadratic.sub_assign(&rhs.quadratic);
        self.higher_order.sub_assign(&rhs.higher_order);
        Ok(())
    }
}

impl SubAssign<f64> for Expression {
    fn sub_assign(&mut self, rhs: f64) {
        self.constant.sub_assign(rhs);
    }
}

#[pymethods]
impl Expression {
    fn __add__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(v) = other.extract::<f64>(py) {
            Ok(self.add(v))
        } else if let Ok(v) = &other.extract::<Expression>(py) {
            self.add(v)
                .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(v) = other.extract::<f64>(py) {
            Ok(AddAssign::add_assign(self, v))
        } else if let Ok(v) = &other.extract::<Expression>(py) {
            FailableAddAssign::add_assign(self, v)
                .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn __sub__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
        if let Ok(v) = other.extract::<f64>(py) {
            Ok(self.sub(v))
        } else if let Ok(v) = &other.extract::<Expression>(py) {
            self.sub(v)
                .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn __isub__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(v) = other.extract::<f64>(py) {
            Ok(SubAssign::sub_assign(self, v))
        } else if let Ok(v) = &other.extract::<Expression>(py) {
            FailableSubAssign::sub_assign(self, v)
                .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }

    fn to_string(&self, environment: &Environment) -> String {
        self.as_string(environment)
    }

    // todo: for multiplications we require the environment, except for
    // multiplication with a scaler value. => Change the input to default
    // to a tuple containing the environment as the second element.
    // On the Python side we need to change the implementation such that
    // the environment is an optional parameter and is injected by default
    // with the global default environment.
    // fn __mul__(&self, py: Python, other: PyObject) -> PyResult<Expression> {
    //     let (unkown, env) = other;
    //     if let Ok(v) = unkown.extract::<f64>(py) {
    //         Ok(self.mul(v))
    //     } else if let Ok(v) = &unkown.extract::<VarRef>(py) {
    //         self.mul((v, env))
    //             .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
    //     } else if let Ok(_) = &unkown.extract::<Expression>(py) {
    //         unimplemented!()
    //     } else {
    //         Err(PyRuntimeError::new_err("unsopported type for operation"))
    //     }
    // }

    fn multiply(&self, py: Python, value: PyObject, env: &Environment) -> PyResult<Expression> {
        if let Ok(v) = value.extract::<f64>(py) {
            Ok(self.mul(v))
        } else if let Ok(v) = &value.extract::<VarRef>(py) {
            self.mul((v, env))
                .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
        } else if let Ok(expr) = &value.extract::<Expression>(py) {
            self.mul((expr, env))
                .map_err(|e| DifferentEnvsException::new_err(e.to_string()))
        } else {
            Err(PyRuntimeError::new_err("unsopported type for operation"))
        }
    }
}

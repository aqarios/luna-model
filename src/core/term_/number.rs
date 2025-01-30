use std::{
    fmt::Display,
    ops::{Add, Sub},
};

#[cfg(feature = "py")]
use pyo3::prelude::*;

use super::{TermAddition, TermMultiplication, TermSubtraction};

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Number {
    pub value: f64,
}

impl Number {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn empty() -> Self {
        Self { value: 0.0 }
    }
}

impl PartialEq<f64> for Number {
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}

impl PartialOrd<f64> for Number {
    fn lt(&self, other: &f64) -> bool {
        self.value < *other
    }
    fn le(&self, other: &f64) -> bool {
        self.value <= *other
    }
    fn gt(&self, other: &f64) -> bool {
        self.value > *other
    }
    fn ge(&self, other: &f64) -> bool {
        self.value >= *other
    }

    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        Some(self.value.partial_cmp(other))?
    }
}

impl TermAddition<f64> for Number {
    fn add_assign(&mut self, rhs: &f64) {
        self.value += rhs
    }
}

impl TermAddition<Number> for Number {
    fn add_assign(&mut self, rhs: &Number) {
        self.value += rhs.value
    }
}

impl TermSubtraction<f64> for Number {
    fn sub_assign(&mut self, rhs: &f64) {
        self.value -= rhs
    }
}

impl TermSubtraction<Number> for Number {
    fn sub_assign(&mut self, rhs: &Number) {
        self.value -= rhs.value
    }
}

impl TermMultiplication<f64> for Number {
    fn mul_assign(&mut self, rhs: &f64) {
        self.value *= rhs
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        Number::new(self.value - rhs.value)
    }
}

impl Sub<&Number> for &Number {
    type Output = Number;
    fn sub(self, rhs: &Number) -> Self::Output {
        Number::new(self.value - rhs.value)
    }
}

impl Add<&Number> for &Number {
    type Output = Number;

    fn add(self, rhs: &Number) -> Self::Output {
        Number {
            value: self.value + rhs.value,
        }
    }
}

impl Add<f64> for Number {
    type Output = Number;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            value: self.value + rhs,
        }
    }
}

// impl AddAssign<f64> for Number {
//     fn add_assign(&mut self, rhs: f64) {
//         self.value += rhs
//     }
// }

// impl AddAssign<f64> for Number {
//     fn add_assign(&mut self, rhs: f64) {
//         self.value += rhs
//     }
// }

// impl AddAssign<Number> for Number {
//     fn add_assign(&mut self, rhs: Number) {
//         self.value += rhs.value
//     }
// }

// impl SubAssign<f64> for Number {
//     fn sub_assign(&mut self, rhs: f64) {
//         self.value -= rhs
//     }
// }

// impl SubAssign for Number {
//     fn sub_assign(&mut self, rhs: Number) {
//         self.value -= rhs.value
//     }
// }
//
// impl SubAssign<&Number> for Number {
//     fn sub_assign(&mut self, rhs: &Number) {
//         self.value -= rhs.value
//     }
// }

// impl Mul<f64> for Number {
//     type Output = f64;
//     fn mul(self, rhs: f64) -> Self::Output {
//         self.value * rhs
//     }
// }
//
// impl MulAssign<f64> for Number {
//     fn mul_assign(&mut self, rhs: f64) {
//         self.value *= rhs
//     }
// }

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

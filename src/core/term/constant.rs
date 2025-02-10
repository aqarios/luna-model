use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[cfg(feature = "py")]
use pyo3::prelude::*;

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Constant {
    pub value: Option<f64>,
}

impl Constant {
    pub fn new(value: f64) -> Self {
        Self { value: Some(value) }
    }

    pub fn reset(&mut self) {
        self.value = None
    }

    pub fn new_from_option(value: Option<f64>) -> Self {
        Self { value }
    }

    pub fn empty() -> Self {
        Self { value: None }
    }

    pub fn as_string(&self) -> String {
        match self.value {
            Some(v) => {
                if v < 0.0 {
                    format!("{}", -v)
                } else if v > 0.0 {
                    format!("{}", v)
                } else {
                    String::from("")
                }
            }
            None => String::from(""),
        }
    }
}

impl Mul<f64> for &Constant {
    type Output = Constant;

    fn mul(self, rhs: f64) -> Self::Output {
        match self.value {
            Some(v) => Constant::new(v * rhs),
            None => Constant::empty(),
        }
    }
}

impl Mul<&Constant> for &Constant {
    type Output = Constant;

    fn mul(self, rhs: &Constant) -> Self::Output {
        match (self.value, rhs.value) {
            (Some(l), Some(r)) => Constant::new(l * r),
            (_, _) => Constant::empty(),
        }
    }
}

impl Add<f64> for &Constant {
    type Output = Constant;

    fn add(self, rhs: f64) -> Self::Output {
        let new_value = match self.value {
            Some(v) => v + rhs,
            None => rhs,
        };
        Constant::new_from_option(Some(new_value))
    }
}

impl Sub<f64> for &Constant {
    type Output = Constant;

    fn sub(self, rhs: f64) -> Self::Output {
        let new_value = match self.value {
            Some(v) => v - rhs,
            None => -rhs,
        };
        Constant::new_from_option(Some(new_value))
    }
}

impl AddAssign<f64> for Constant {
    fn add_assign(&mut self, rhs: f64) {
        match self.value {
            Some(v) => _ = self.value.insert(v + rhs),
            None => _ = self.value.insert(rhs),
        }
    }
}

impl SubAssign<f64> for Constant {
    fn sub_assign(&mut self, rhs: f64) {
        match self.value {
            Some(v) => _ = self.value.insert(v - rhs),
            None => _ = self.value.insert(-rhs),
        }
    }
}

impl MulAssign<f64> for Constant {
    fn mul_assign(&mut self, rhs: f64) {
        match self.value {
            Some(v) => _ = self.value.insert(v * rhs),
            None => (),
        }
    }
}

impl Add<&Constant> for &Constant {
    type Output = Constant;

    fn add(self, rhs: &Constant) -> Self::Output {
        Constant::new_from_option(add_options(self.value, rhs.value))
    }
}

impl AddAssign<&Constant> for Constant {
    fn add_assign(&mut self, rhs: &Constant) {
        match (self.value, rhs.value) {
            (None, Some(r)) => _ = self.value.insert(r),
            (Some(s), Some(r)) => _ = self.value.insert(s + r),
            (_, _) => (),
        }
    }
}

impl Sub<&Constant> for &Constant {
    type Output = Constant;

    fn sub(self, rhs: &Constant) -> Self::Output {
        Constant::new_from_option(sub_options(self.value, rhs.value))
    }
}

impl SubAssign<&Constant> for Constant {
    fn sub_assign(&mut self, rhs: &Constant) {
        match (self.value, rhs.value) {
            (None, Some(r)) => _ = self.value.insert(-r),
            (Some(s), Some(r)) => _ = self.value.insert(s - r),
            (_, _) => (),
        }
    }
}

fn add_options<T: Add<Output = T>>(lhs: Option<T>, rhs: Option<T>) -> Option<T> {
    match (lhs, rhs) {
        (None, None) => None,
        (Some(s), None) => Some(s),
        (None, Some(r)) => Some(r),
        (Some(s), Some(r)) => Some(s + r),
    }
}

fn sub_options<T: Sub<Output = T> + Neg<Output = T>>(lhs: Option<T>, rhs: Option<T>) -> Option<T> {
    match (lhs, rhs) {
        (None, None) => None,
        (Some(s), None) => Some(s),
        (None, Some(r)) => Some(-r),
        (Some(s), Some(r)) => Some(s - r),
    }
}

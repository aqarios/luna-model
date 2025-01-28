use crate::core::varref::VarId;

use super::number::Number;
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, AddAssign},
};

#[cfg(feature = "py")]
use pyo3::prelude::*;

// type VariableKey = usize;

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone)]
pub struct Linear {
    pub variables: HashMap<VarId, Number>,
}

impl Linear {
    pub fn new(varid: VarId, value: Option<f64>) -> Self {
        let mut lin = Self::empty();
        lin.variables
            .insert(varid, Number::new(value.unwrap_or(1.0)));
        lin
    }

    pub fn empty() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    fn from(other: &Self) -> Self {
        Self {
            variables: other.variables.clone(),
        }
    }

    // pub fn mul_var(&mut self, var: &Variable, value: f64) {
    //     let varkey = var.key();
    //     match self.variables.get_mut(varkey) {
    //         Some(e) => *e *= value,
    //         None => {
    //             _ = self
    //                 .variables
    //                 .insert(varkey.to_string(), Number::new(value));
    //             ()
    //         }
    //     }
    // }

    // pub fn add_var(&mut self, var: &Variable) {
    //     let varkey = var.key();
    //     match self.variables.get_mut(varkey) {
    //         Some(e) => *e += 1.0,
    //         None => {
    //             _ = self.variables.insert(varkey.to_string(), Number::new(1.0));
    //             ()
    //         }
    //     }
    // }
}

impl Add<Linear> for Linear {
    type Output = Linear;
    fn add(self, rhs: Linear) -> Self::Output {
        let mut lin = Linear::from(&self);

        for (k, v) in rhs.variables.iter() {
            match lin.variables.get_mut(k) {
                Some(e) => *e += v.value,
                None => {
                    _ = lin.variables.insert(*k, Number::new(v.value));
                    ()
                }
            }
        }

        lin
    }
}

impl Add<&Linear> for &Linear {
    type Output = Linear;
    fn add(self, rhs: &Linear) -> Self::Output {
        let mut lin = Linear::from(&self);

        for (k, v) in rhs.variables.iter() {
            match lin.variables.get_mut(k) {
                Some(e) => *e += v.value,
                None => {
                    _ = lin.variables.insert(*k, Number::new(v.value));
                    ()
                }
            }
        }

        lin
    }
}

impl AddAssign<Linear> for Linear {
    fn add_assign(&mut self, rhs: Linear) {
        for (k, v) in rhs.variables.iter() {
            match self.variables.get_mut(k) {
                Some(e) => *e += v.value,
                None => {
                    _ = self.variables.insert(*k, Number::new(v.value));
                    ()
                }
            }
        }
    }
}

// impl AddAssign<Variable> for Linear {
//     fn add_assign(&mut self, rhs: Variable) {
//         let varkey = rhs.key();
//         let is_contained = self.variables.contains_key(&varkey);
//         match is_contained {
//             true => unimplemented!(),
//             false => self.variables.insert(varkey, Number::new(1.0)),
//         };
//     }
// }
//
// impl AddAssign<&Variable> for Linear {
//     fn add_assign(&mut self, rhs: &Variable) {
//         let varkey = rhs.key();
//         let is_contained = self.variables.contains_key(&varkey);
//         match is_contained {
//             true => unimplemented!(),
//             false => self.variables.insert(varkey, Number::new(1.0)),
//         };
//     }
// }

impl Display for Linear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self
            .variables
            .iter()
            .map(|(k, v)| format!("{} * {}", v, k))
            .collect::<Vec<String>>()
            .join(" + ");

        write!(f, "{}", output)
    }
}

// impl<'py> FromPyObject<'py> for Linear {
//     fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
//         println!("in extract for linear");
//         let variables: HashMap<VarId, Number> = ob.extract()?;
//         Ok(Self { variables })
//     }
// }

// impl Add for Linear {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         Self {}
//     }
// }
//
// impl Add<Constant> for Linear {
//     type Output = Self;
//
//     fn add(self, rhs: Constant) -> Self::Output {
//         Self {}
//     }
// }
//
// impl Mul<Constant> for Linear {
//     type Output = Self;
//
//     fn mul(self, rhs: Constant) -> Self::Output {
//         Self {}
//     }
// }
//
// impl Sub for Linear {
//     type Output = Self;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         Self {}
//     }
// }

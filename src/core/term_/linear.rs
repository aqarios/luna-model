use crate::core::varref::{VarId, VarRef, DEFAULT_SCALER_VALUE};

use super::{
    number::Number,
    operations::{TermAddition, TermCreation, TermMultiplication, TermSubtraction},
};
use std::{collections::HashMap, fmt::Display};

#[cfg(feature = "py")]
use pyo3::prelude::*;

// type VariableKey = usize;

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq)]
pub struct Linear {
    pub variables: HashMap<VarId, f64>,
}

impl Linear {
    pub fn from_vars(a: &VarRef, b: (&VarRef, Option<f64>)) -> Self {
        let mut linear = Linear::new(b.0, b.1);
        linear.add_assign(a);
        linear
    }

    pub fn new(var: &VarRef, value: Option<f64>) -> Self {
        let mut lin = Self::empty();
        lin.variables
            .insert(var.id, value.unwrap_or(DEFAULT_SCALER_VALUE));
        lin
    }

    pub fn empty() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

impl TermCreation for Linear {
    fn from_other(other: &Self) -> Self {
        Self {
            variables: other.variables.clone(),
        }
    }
}

impl TermAddition<Linear> for Linear {
    fn add_assign(&mut self, rhs: &Linear) {
        for (k, v) in rhs.variables.iter() {
            match self.variables.get_mut(k) {
                Some(e) => {
                    e.add_assign(&v.value);
                    if *e == 0.0 {
                        self.variables.remove(k);
                    }
                }
                None => {
                    _ = self.variables.insert(*k, Number::new(v.value));
                    ()
                }
            }
        }
    }
}

impl TermAddition<VarRef> for Linear {
    fn add_assign(&mut self, rhs: &VarRef) {
        match self.variables.get_mut(&rhs.id) {
            Some(e) => {
                e.add_assign(&DEFAULT_SCALER_VALUE);
                if *e == 0.0 {
                    self.variables.remove(&rhs.id);
                }
            }
            None => {
                _ = self
                    .variables
                    .insert(rhs.id, Number::new(DEFAULT_SCALER_VALUE));
                ()
            }
        }
    }
}

impl TermSubtraction<Linear> for Linear {
    fn sub_assign(&mut self, rhs: &Linear) {
        for (k, v) in rhs.variables.iter() {
            match self.variables.get_mut(k) {
                Some(e) => {
                    e.sub_assign(&v.value);
                    if *e == 0.0 {
                        self.variables.remove(k);
                    }
                }
                None => {
                    _ = self.variables.insert(*k, Number::new(-v.value));
                    ()
                }
            }
        }
    }
}

impl TermSubtraction<VarRef> for Linear {
    fn sub_assign(&mut self, rhs: &VarRef) {
        match self.variables.get_mut(&rhs.id) {
            Some(e) => {
                e.sub_assign(&DEFAULT_SCALER_VALUE);
                if *e == 0.0 {
                    self.variables.remove(&rhs.id);
                }
            }
            None => {
                _ = self
                    .variables
                    .insert(rhs.id, Number::new(DEFAULT_SCALER_VALUE));
                ()
            }
        }
    }
}

impl TermMultiplication<f64> for Linear {
    fn mul_assign(&mut self, rhs: &f64) {
        for (_, v) in self.variables.iter_mut() {
            v.mul_assign(rhs);
        }
    }
}

// impl Add<Linear> for Linear {
//     type Output = Linear;
//     fn add(self, rhs: Linear) -> Self::Output {
//         let mut lin = Linear::from(&self);
//
//         for (k, v) in rhs.variables.iter() {
//             match lin.variables.get_mut(k) {
//                 Some(e) => *e += v.value,
//                 None => {
//                     _ = lin.variables.insert(*k, Number::new(v.value));
//                     ()
//                 }
//             }
//         }
//
//         lin
//     }
// }
//
// impl Add<&Linear> for &Linear {
//     type Output = Linear;
//     fn add(self, rhs: &Linear) -> Self::Output {
//         let mut lin = Linear::from(&self);
//
//         for (k, v) in rhs.variables.iter() {
//             match lin.variables.get_mut(k) {
//                 Some(e) => *e += v.value,
//                 None => {
//                     _ = lin.variables.insert(*k, Number::new(v.value));
//                     ()
//                 }
//             }
//         }
//
//         lin
//     }
// }
//
// impl AddAssign<Linear> for Linear {
//     fn add_assign(&mut self, rhs: Linear) {
//         for (k, v) in rhs.variables.iter() {
//             match self.variables.get_mut(k) {
//                 Some(e) => *e += v.value,
//                 None => {
//                     _ = self.variables.insert(*k, Number::new(v.value));
//                     ()
//                 }
//             }
//         }
//     }
// }

// impl AddAssign<VarRef> for Linear {
//     fn add_assign(&mut self, rhs: VarRef) {
//         match self.variables.get_mut(&rhs.id) {
//             Some(e) => *e += DEFAULT_VAR_VALUE,
//             None => {
//                 _ = self.variables.insert(rhs.id, Number::new(DEFAULT_VAR_VALUE));
//                 ()
//             }
//         }
//     }
// }

// impl AddAssign<&VarRef> for Linear {
//     fn add_assign(&mut self, rhs: &VarRef) {
//         match self.variables.get_mut(&rhs.id) {
//             Some(e) => *e += DEFAULT_VAR_VALUE,
//             None => {
//                 _ = self.variables.insert(rhs.id, Number::new(DEFAULT_VAR_VALUE));
//                 ()
//             }
//         }
//     }
// }

// impl Sub<Linear> for Linear {
//     type Output = Linear;
//     fn sub(self, rhs: Linear) -> Self::Output {
//         let mut lin = Linear::from_other(&self);
//
//         for (k, v) in rhs.variables.iter() {
//             match lin.variables.get_mut(k) {
//                 Some(e) => *e -= v.value,
//                 None => {
//                     _ = lin.variables.insert(*k, Number::new(-v.value));
//                     ()
//                 }
//             }
//         }
//
//         lin
//     }
// }
//
// impl Sub<&Linear> for &Linear {
//     type Output = Linear;
//     fn sub(self, rhs: &Linear) -> Self::Output {
//         let mut lin = Linear::from_other(&self);
//
//         for (k, v) in rhs.variables.iter() {
//             match lin.variables.get_mut(k) {
//                 Some(e) => {
//                     *e -= v.value;
//                     if *e == 0.0 {
//                         lin.variables.remove(k);
//                     }
//                 }
//                 None => {
//                     _ = lin.variables.insert(*k, Number::new(-v.value));
//                     ()
//                 }
//             }
//         }
//
//         lin
//     }
// }
//
// impl SubAssign<VarRef> for Linear {
//     fn sub_assign(&mut self, rhs: VarRef) {
//         match self.variables.get_mut(&rhs.id) {
//             Some(e) => {
//                 *e -= DEFAULT_VAR_VALUE;
//                 if *e == 0.0 {
//                     self.variables.remove(&rhs.id);
//                 }
//             }
//             None => {
//                 _ = self
//                     .variables
//                     .insert(rhs.id, Number::new(-DEFAULT_VAR_VALUE));
//                 ()
//             }
//         }
//     }
// }
//
// impl SubAssign<&VarRef> for Linear {
//     fn sub_assign(&mut self, rhs: &VarRef) {
//         match self.variables.get_mut(&rhs.id) {
//             Some(e) => {
//                 *e -= DEFAULT_VAR_VALUE;
//                 if *e == 0.0 {
//                     self.variables.remove(&rhs.id);
//                 }
//             }
//             None => {
//                 _ = self
//                     .variables
//                     .insert(rhs.id, Number::new(-DEFAULT_VAR_VALUE));
//                 ()
//             }
//         }
//     }
// }

//impl SubAssign<Linear> for Linear {
//    fn sub_assign(&mut self, rhs: Linear) {
//        for (k, v) in rhs.variables.iter() {
//            match self.variables.get_mut(k) {
//                Some(e) => {
//                    *e -= v.value;
//                    if *e == 0.0 {
//                        self.variables.remove(k);
//                    }
//                }
//                None => {
//                    _ = self.variables.insert(*k, Number::new(-v.value));
//                    ()
//                }
//            }
//        }
//    }
//}
//
//impl SubAssign<&Linear> for Linear {
//    fn sub_assign(&mut self, rhs: &Linear) {
//        for (k, v) in rhs.variables.iter() {
//            match self.variables.get_mut(k) {
//                Some(e) => {
//                    *e -= v.value;
//                    if *e == 0.0 {
//                        self.variables.remove(k);
//                    }
//                }
//                None => {
//                    _ = self.variables.insert(*k, Number::new(-v.value));
//                    ()
//                }
//            }
//        }
//    }
//}
// impl AddAssign<Variable> for Linear {
//     fn add_assign(&mut self, rhs: Variable) {
//         let varkey = rhs.key();
//         let is_contained = self.variables.contains_key(&varkey);
//         match is_contained {
//             true => unimplemented!(),
//             false => self.variables.insert(varkey, Number::new(DEFAULT_VAR_VALUE)),
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
//             false => self.variables.insert(varkey, Number::new(DEFAULT_VAR_VALUE)),
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

use std::collections::HashMap;

#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::{
    environment::EnvId,
    operations::{Term, TermAddition, TermSubtraction},
    Environment,
};

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq)]
pub struct HigherOrder {
    pub env_id: EnvId,
    variables: Option<HashMap<u64, f64>>,
}

impl HigherOrder {
    pub fn empty(env_id: EnvId) -> Self {
        Self {
            env_id,
            variables: None,
        }
    }

    pub fn as_string(&self, _environment: &Environment) -> String {
        String::from("")
    }
}

impl Term<u64> for HigherOrder {
    fn new_from_other(other: &Self) -> Self {
        Self {
            env_id: other.env_id,
            variables: other.variables.clone(),
        }
    }

    fn has_variables(&self) -> bool {
        self.variables.is_some()
    }

    fn mutable_variables(&mut self) -> &mut HashMap<u64, f64> {
        self.variables.as_mut().unwrap()
    }

    fn variables(&self) -> &HashMap<u64, f64> {
        self.variables.as_ref().unwrap()
    }

    fn fill_variables(&mut self, variables: HashMap<u64, f64>) -> &mut HashMap<u64, f64> {
        self.variables.insert(variables)
    }
}

impl TermAddition<u64> for HigherOrder {}
impl TermSubtraction<u64> for HigherOrder {}

// impl Add<&HigherOrder> for &HigherOrder {
//     type Output = HigherOrder;
//
//     fn add(self, rhs: &HigherOrder) -> Self::Output {
//         // If the `self` variables are not present we can directly return a copy
//         // of the `rhs` variables as a new higher_order term.
//         if self.variables.is_none() {
//             return HigherOrder::new_from_other(&rhs);
//         }
//         // From here, we know that `self` contains values.
//         // If the `rhs` variables are not present we can directly return a copy
//         // of the `self` variables as a new higher_order term.
//         if rhs.variables.is_none() {
//             return HigherOrder::new_from_other(&self);
//         }
//         // Now both `self.variables` and `rhs.variables` have values.
//         // So we can start from either the `self` or the `rhs` term.
//         // We choose the `self` term here.
//         let mut out = HigherOrder::new_from_other(&self);
//         let out_vars = out.variables.as_mut().unwrap();
//         // We can now insert the values from `rhs`.
//         for (key, value) in rhs.variables.as_ref().unwrap().iter() {
//             match out_vars.get_mut(key) {
//                 Some(e) => {
//                     e.add_assign(value);
//                     if *e == 0.0 {
//                         out_vars.remove(key);
//                     }
//                 }
//                 None => _ = out_vars.insert(*key, *value),
//             }
//         }
//         out
//     }
// }
//
// impl AddAssign<&HigherOrder> for HigherOrder {
//     fn add_assign(&mut self, rhs: &HigherOrder) {
//         // If other value does not contain variables than we do not need
//         // to do anything. Current higher_order term stays as is.
//         if rhs.variables.is_none() {
//             return;
//         }
//
//         // We need to insert the rhs variables into self and the
//         // current self does not contain any values itself.
//         if self.variables.is_none() {
//             let vars = rhs.variables.as_ref().unwrap();
//             let _ = self.variables.insert(vars.clone());
//             return;
//         }
//         // Now we know that both `self.variables` and `rhs.variables`
//         // contain values. We need to merge them using the add operation.
//         // mutable variables of self (mutable reference).
//         let selfvars = self.variables.as_mut().unwrap();
//         for (key, value) in rhs.variables.as_ref().unwrap().iter() {
//             match selfvars.get_mut(key) {
//                 Some(e) => {
//                     e.add_assign(value);
//                     if *e == 0.0 {
//                         selfvars.remove(key);
//                     }
//                 }
//                 None => _ = selfvars.insert(*key, *value),
//             }
//         }
//     }
// }
//
// impl Sub<&HigherOrder> for &HigherOrder {
//     type Output = HigherOrder;
//
//     fn sub(self, rhs: &HigherOrder) -> Self::Output {
//         // If the `self` variables are not present we can directly return a copy
//         // of the `rhs` variables as a new higher_order term.
//         // We subtract the current (`self`) higher_order term for `0`. Thus we need the
//         // sign flipped for all values.
//         if self.variables.is_none() {
//             let mut out = HigherOrder::new_from_other(&rhs);
//             // We subtract the current (`self`) higher_order term for `0`. Thus we need the
//             // sign flipped for all values, i.e., multiply each value by `-1`.
//             // todo: is there something faster/better to achieve this??
//             out.variables
//                 .as_mut()
//                 .unwrap()
//                 .iter_mut()
//                 .for_each(|(_, value)| {
//                     value.mul_assign(-1.0);
//                 });
//         }
//         // From here, we know that `self` contains values.
//         // If the `rhs` variables are not present we can directly return a copy
//         // of the `self` variables as a new higher_order term.
//         // Basically we subtract `0` from the current (`self`) higher_order term.
//         if rhs.variables.is_none() {
//             return HigherOrder::new_from_other(&self);
//         }
//         // Now both `self.variables` and `rhs.variables` have values.
//         // So we can start from either the `self` or the `rhs` term.
//         // We choose the `self` term here.
//         // self - rhs
//         let mut out = HigherOrder::new_from_other(&self);
//         let out_vars = out.variables.as_mut().unwrap();
//         // We can now insert the values from `rhs`.
//         for (key, value) in rhs.variables.as_ref().unwrap().iter() {
//             match out_vars.get_mut(key) {
//                 Some(e) => {
//                     e.sub_assign(value);
//                     if *e == 0.0 {
//                         out_vars.remove(key);
//                     }
//                 }
//                 None => _ = out_vars.insert(*key, *value),
//             }
//         }
//         out
//     }
// }
//
// impl SubAssign<&HigherOrder> for HigherOrder {
//     fn sub_assign(&mut self, rhs: &HigherOrder) {
//         // If other value does not contain variables than we do not need
//         // to do anything. Current higher_order term stays as is.
//         // self - rhs = self - 0 = self
//         if rhs.variables.is_none() {
//             return;
//         }
//
//         // We need to insert the rhs variables into self and the
//         // current self does not contain any values itself.
//         // self - rhs = 0 - rhs = - rhs;
//         if self.variables.is_none() {
//             let vars = rhs.variables.as_ref().unwrap();
//             let selfvars = self.variables.insert(vars.clone());
//             selfvars.iter_mut().for_each(|(_, value)| {
//                 value.mul_assign(-1.0);
//             });
//             return;
//         }
//         // Now we know that both `self.variables` and `rhs.variables`
//         // contain values. We need to merge them using the add operation.
//         // mutable variables of self (mutable reference).
//         let selfvars = self.variables.as_mut().unwrap();
//         for (key, value) in rhs.variables.as_ref().unwrap().iter() {
//             match selfvars.get_mut(key) {
//                 Some(e) => {
//                     e.sub_assign(value);
//                     if *e == 0.0 {
//                         selfvars.remove(key);
//                     }
//                 }
//                 None => _ = selfvars.insert(*key, *value),
//             }
//         }
//     }
// }

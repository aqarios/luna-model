use std::{
    collections::HashMap,
    ops::{Add, AddAssign, MulAssign, Sub, SubAssign},
};

#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::{environment::EnvId, Environment};

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq)]
pub struct Quadratic {
    pub env_id: EnvId,
    variables: Option<HashMap<u64, f64>>,
}

impl Quadratic {
    pub fn empty(env_id: EnvId) -> Self {
        Self {
            env_id,
            variables: None,
        }
    }

    pub fn new_from_other(other: &Self) -> Self {
        Self {
            env_id: other.env_id,
            variables: other.variables.clone(),
        }
    }

    pub fn as_string(&self, environment: &Environment) -> String {
        match &self.variables {
            Some(vs) => {
                vs.iter()
                    .map(|(key, value)| {
                        let (a, b) = Self::get_key_contributions(key);
                        let var_a = environment.variables.get(a as usize).unwrap();
                        let var_b = environment.variables.get(b as usize).unwrap();
                        if *value < 0.0 {
                            format!("- {} * {} * {}", -value, var_a.name, var_b.name)
                        } else {
                            format!("+ {} * {} * {}", value, var_a.name, var_b.name)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                String::from("todo")
            }
            None => String::from(""),
        }
    }

    pub fn make_key(a: u32, b: u32) -> u64 {
        // The larger key is always at the end.
        if a < b {
            let au64 = (a as u64) << 32;
            au64 | (b as u64)
        } else if a > b {
            let bu64 = (b as u64) << 32;
            bu64 | (a as u64)
        } else {
            panic!("equal key")
        }
    }

    pub fn get_key_contributions(key: &u64) -> (u32, u32) {
        ((*key >> 32) as u32, *key as u32)
    }
}

impl Add<&Quadratic> for &Quadratic {
    type Output = Quadratic;

    fn add(self, rhs: &Quadratic) -> Self::Output {
        // If the `self` variables are not present we can directly return a copy
        // of the `rhs` variables as a new quadratic term.
        if self.variables.is_none() {
            return Quadratic::new_from_other(&rhs);
        }
        // From here, we know that `self` contains values.
        // If the `rhs` variables are not present we can directly return a copy
        // of the `self` variables as a new quadratic term.
        if rhs.variables.is_none() {
            return Quadratic::new_from_other(&self);
        }
        // Now both `self.variables` and `rhs.variables` have values.
        // So we can start from either the `self` or the `rhs` term.
        // We choose the `self` term here.
        let mut out = Quadratic::new_from_other(&self);
        let out_vars = out.variables.as_mut().unwrap();
        // We can now insert the values from `rhs`.
        for (key, value) in rhs.variables.as_ref().unwrap().iter() {
            match out_vars.get_mut(key) {
                Some(e) => {
                    e.add_assign(value);
                    if *e == 0.0 {
                        out_vars.remove(key);
                    }
                }
                None => _ = out_vars.insert(*key, *value),
            }
        }
        out
    }
}

impl AddAssign<&Quadratic> for Quadratic {
    fn add_assign(&mut self, rhs: &Quadratic) {
        // If other value does not contain variables than we do not need
        // to do anything. Current quadratic term stays as is.
        if rhs.variables.is_none() {
            return;
        }

        // We need to insert the rhs variables into self and the
        // current self does not contain any values itself.
        if self.variables.is_none() {
            let vars = rhs.variables.as_ref().unwrap();
            let _ = self.variables.insert(vars.clone());
            return;
        }
        // Now we know that both `self.variables` and `rhs.variables`
        // contain values. We need to merge them using the add operation.
        // mutable variables of self (mutable reference).
        let selfvars = self.variables.as_mut().unwrap();
        for (key, value) in rhs.variables.as_ref().unwrap().iter() {
            match selfvars.get_mut(key) {
                Some(e) => {
                    e.add_assign(value);
                    if *e == 0.0 {
                        selfvars.remove(key);
                    }
                }
                None => _ = selfvars.insert(*key, *value),
            }
        }
    }
}

impl Sub<&Quadratic> for &Quadratic {
    type Output = Quadratic;

    fn sub(self, rhs: &Quadratic) -> Self::Output {
        // If the `self` variables are not present we can directly return a copy
        // of the `rhs` variables as a new quadratic term.
        // We subtract the current (`self`) quadratic term for `0`. Thus we need the
        // sign flipped for all values.
        if self.variables.is_none() {
            let mut out = Quadratic::new_from_other(&rhs);
            // We subtract the current (`self`) quadratic term for `0`. Thus we need the
            // sign flipped for all values, i.e., multiply each value by `-1`.
            // todo: is there something faster/better to achieve this??
            out.variables
                .as_mut()
                .unwrap()
                .iter_mut()
                .for_each(|(_, value)| {
                    value.mul_assign(-1.0);
                });
        }
        // From here, we know that `self` contains values.
        // If the `rhs` variables are not present we can directly return a copy
        // of the `self` variables as a new quadratic term.
        // Basically we subtract `0` from the current (`self`) quadratic term.
        if rhs.variables.is_none() {
            return Quadratic::new_from_other(&self);
        }
        // Now both `self.variables` and `rhs.variables` have values.
        // So we can start from either the `self` or the `rhs` term.
        // We choose the `self` term here.
        // self - rhs
        let mut out = Quadratic::new_from_other(&self);
        let out_vars = out.variables.as_mut().unwrap();
        // We can now insert the values from `rhs`.
        for (key, value) in rhs.variables.as_ref().unwrap().iter() {
            match out_vars.get_mut(key) {
                Some(e) => {
                    e.sub_assign(value);
                    if *e == 0.0 {
                        out_vars.remove(key);
                    }
                }
                None => _ = out_vars.insert(*key, *value),
            }
        }
        out
    }
}

impl SubAssign<&Quadratic> for Quadratic {
    fn sub_assign(&mut self, rhs: &Quadratic) {
        // If other value does not contain variables than we do not need
        // to do anything. Current quadratic term stays as is.
        // self - rhs = self - 0 = self
        if rhs.variables.is_none() {
            return;
        }

        // We need to insert the rhs variables into self and the
        // current self does not contain any values itself.
        // self - rhs = 0 - rhs = - rhs;
        if self.variables.is_none() {
            let vars = rhs.variables.as_ref().unwrap();
            let selfvars = self.variables.insert(vars.clone());
            selfvars.iter_mut().for_each(|(_, value)| {
                value.mul_assign(-1.0);
            });
            return;
        }
        // Now we know that both `self.variables` and `rhs.variables`
        // contain values. We need to merge them using the add operation.
        // mutable variables of self (mutable reference).
        let selfvars = self.variables.as_mut().unwrap();
        for (key, value) in rhs.variables.as_ref().unwrap().iter() {
            match selfvars.get_mut(key) {
                Some(e) => {
                    e.sub_assign(value);
                    if *e == 0.0 {
                        selfvars.remove(key);
                    }
                }
                None => _ = selfvars.insert(*key, *value),
            }
        }
    }
}

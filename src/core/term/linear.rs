use std::{
    collections::HashMap,
    ops::{Add, AddAssign, MulAssign, Sub, SubAssign},
};

#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::{
    environment::{self, EnvId},
    exceptions::VariablesFromDifferentEnvsError,
    variable::VarId,
    Environment, VarRef,
};

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq)]
pub struct Linear {
    pub env_id: EnvId,
    pub variables: Option<HashMap<VarId, f64>>,
}

/// methods used to create a linear term efficiently.
impl Linear {
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

    /// Efficient production of linear term for a single value.
    pub fn new(a: (&VarRef, f64)) -> Self {
        let (a_ref, av) = a;
        let mut variables = HashMap::new();
        variables.insert(a_ref.id, av);
        Self {
            // variables,
            variables: Some(variables),
            env_id: a_ref.env_id,
        }
    }
    /// Linear terms are created when two variables are added or subtracted.
    /// This generates either `a + b` or `a - b`
    /// What if a and b are equal? Then the sum of the passed values are stored.
    pub fn new_from_vars(
        a: (&VarRef, f64),
        b: (&VarRef, f64),
    ) -> Result<Self, VariablesFromDifferentEnvsError> {
        let (a_ref, av) = a;
        let (b_ref, bv) = b;

        if a_ref.env_id != b_ref.env_id {
            return Err(VariablesFromDifferentEnvsError);
        }

        let mut variables = HashMap::new();
        if a_ref.id == b_ref.id {
            variables.insert(a_ref.id, av + bv);
        } else {
            variables.insert(a_ref.id, av);
            variables.insert(b_ref.id, bv);
        }
        Ok(Self {
            variables: Some(variables),
            // variables,
            env_id: a_ref.env_id,
        })
    }

    pub fn as_string(&self, environment: &Environment) -> String {
        match &self.variables {
            Some(vs) => vs
                .iter()
                .map(|(key, value)| {
                    let var = environment.get(key);
                    if *value == 1.0 {
                        format!("{}", var.name)
                    } else if *value < 0.0 {
                        format!("{} * {}", -value, var.name)
                    } else {
                        format!("{} * {}", value, var.name)
                    }
                })
                .collect::<Vec<String>>()
                .join(" + "),
            None => String::from(""),
        }
    }
}

impl Add<&Linear> for &Linear {
    type Output = Linear;

    fn add(self, rhs: &Linear) -> Self::Output {
        // If the `self` variables are not present we can directly return a copy
        // of the `rhs` variables as a new linear term.
        if self.variables.is_none() {
            return Linear::new_from_other(&rhs);
        }
        // From here, we know that `self` contains values.
        // If the `rhs` variables are not present we can directly return a copy
        // of the `self` variables as a new linear term.
        if rhs.variables.is_none() {
            return Linear::new_from_other(&self);
        }
        // Now both `self.variables` and `rhs.variables` have values.
        // So we can start from either the `self` or the `rhs` term.
        // We choose the `self` term here.
        let mut out = Linear::new_from_other(&self);
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

impl AddAssign<&Linear> for Linear {
    fn add_assign(&mut self, rhs: &Linear) {
        // If other value does not contain variables than we do not need
        // to do anything. Current linear term stays as is.
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

impl Sub<&Linear> for &Linear {
    type Output = Linear;

    fn sub(self, rhs: &Linear) -> Self::Output {
        // If the `self` variables are not present we can directly return a copy
        // of the `rhs` variables as a new linear term.
        // We subtract the current (`self`) linear term for `0`. Thus we need the
        // sign flipped for all values.
        if self.variables.is_none() {
            let mut out = Linear::new_from_other(&rhs);
            // We subtract the current (`self`) linear term for `0`. Thus we need the
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
        // of the `self` variables as a new linear term.
        // Basically we subtract `0` from the current (`self`) linear term.
        if rhs.variables.is_none() {
            return Linear::new_from_other(&self);
        }
        // Now both `self.variables` and `rhs.variables` have values.
        // So we can start from either the `self` or the `rhs` term.
        // We choose the `self` term here.
        // self - rhs
        let mut out = Linear::new_from_other(&self);
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

impl SubAssign<&Linear> for Linear {
    fn sub_assign(&mut self, rhs: &Linear) {
        // If other value does not contain variables than we do not need
        // to do anything. Current linear term stays as is.
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

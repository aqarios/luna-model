use crate::core::{
    environment::EnvId,
    exceptions::VariablesFromDifferentEnvsError,
    higher_order_operations::TermVarMultiplicationC,
    operations::{
        Term, TermAddition, TermConstantMultiplication, TermFloatMultiplication, TermSubtraction,
    },
    Environment, VarRef, Vtype,
};
use std::collections::HashMap;

#[cfg(feature = "py")]
use pyo3::prelude::*;

use super::{higher_order::HigherOrderKey, HigherOrder};

pub type QuadraticKey = u64;

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq)]
pub struct Quadratic {
    pub env_id: EnvId,
    variables: Option<HashMap<QuadraticKey, f64>>,
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

    pub fn new_from_vars(a: &VarRef, b: &VarRef) -> Result<Self, VariablesFromDifferentEnvsError> {
        if a.env_id != b.env_id {
            return Err(VariablesFromDifferentEnvsError);
        }

        let key = Self::make_key(a.id, b.id);
        let mut variables = HashMap::new();
        variables.insert(key, 1.0);
        Ok(Self {
            env_id: a.env_id,
            variables: Some(variables),
        })
    }

    pub fn new_from_vars_with_value(a: &u32, b: &VarRef, value: f64) -> Self {
        let key = Self::make_key(*a, b.id);
        let mut variables = HashMap::new();
        variables.insert(key, value);
        Self {
            env_id: b.env_id,
            variables: Some(variables),
        }
    }

    pub fn new_from_keys_with_value(env_id: EnvId, a: &u32, b: &u32, value: f64) -> Self {
        let key = Self::make_key(*a, *b);
        let mut variables = HashMap::new();
        variables.insert(key, value);
        Self {
            env_id,
            variables: Some(variables),
        }
    }

    pub fn as_string(&self, environment: &Environment) -> String {
        match &self.variables {
            Some(vs) => vs
                .iter()
                .map(|(key, value)| {
                    let (a, b) = Self::get_key_contributions(key);
                    let var_a = environment.get(&a);
                    let var_b = environment.get(&b);
                    if *value < 0.0 {
                        format!("- {} * {} * {}", -value, var_a.name, var_b.name)
                    } else if *value == 1.0 {
                        format!("{} * {}", var_a.name, var_b.name)
                    } else {
                        format!("{} * {} * {}", value, var_a.name, var_b.name)
                    }
                })
                .collect::<Vec<String>>()
                .join(" + "),
            None => String::from(""),
        }
    }

    pub fn make_key(a: u32, b: u32) -> QuadraticKey {
        // The larger key is always at the end.
        if a < b {
            let au64 = (a as u64) << 32;
            au64 | (b as u64)
        } else {
            let bu64 = (b as u64) << 32;
            bu64 | (a as u64)
        }
    }

    pub fn get_key_contributions(key: &QuadraticKey) -> (u32, u32) {
        ((*key >> 32) as u32, *key as u32)
    }

    pub fn append(&mut self, other: Option<Self>) {
        match other {
            None => (),
            Some(q) => match self.has_variables() {
                true => match q.has_variables() {
                    true => {
                        let selfvars = self.mutable_variables();
                        for (key, value) in q.variables().iter() {
                            selfvars.insert(*key, *value);
                        }
                    }
                    false => (),
                },
                false => self.variables = q.variables.clone(),
            },
        }
    }

    pub fn append_elem(&mut self, key_a: &u32, key_b: &u32, value: f64) {
        let key = Self::make_key(*key_a, *key_b);
        match self.has_variables() {
            false => {
                let mut nh = HashMap::new();
                nh.insert(key, value);
                self.variables = Some(nh);
            }
            true => {
                self.mutable_variables().insert(key, value);
            }
        }
    }
}

impl Term<QuadraticKey> for Quadratic {
    fn empty(env_id: EnvId) -> Self {
        Self {
            env_id,
            variables: None,
        }
    }
    fn reset(&mut self) {
        self.variables = None
    }

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

    fn env_id(&self) -> EnvId {
        self.env_id
    }
}

impl TermAddition<QuadraticKey> for Quadratic {}
impl TermSubtraction<QuadraticKey> for Quadratic {}
impl TermFloatMultiplication<QuadraticKey> for Quadratic {}
impl TermConstantMultiplication<QuadraticKey> for Quadratic {}

impl TermVarMultiplicationC<QuadraticKey, HigherOrder, HigherOrderKey> for Quadratic {
    fn mul(&self, var: &VarRef, environment: &Environment) -> (Self, Option<HigherOrder>) {
        if !self.has_variables() {
            return (Self::empty(self.env_id), None);
        }
        // We are dealing if a trivial variable here in the sense that it does not have a
        // factor associated yet, i.e., the factor of the variable is 1.0. Thus we can
        // take a lot of shortcuts here that are not directly applicable to the multiplication
        // with a variable from another expression, where the facctor of the variable can be
        // anything.
        //
        // This method is esentially only checking if a new quadratic term is created by
        // the multiplication.
        let mut out = Self::new_from_other(&self);
        let outvars = out.mutable_variables();

        let mut higher_order: Option<HigherOrder> = None;

        for (key, value) in self.variables().iter() {
            let (a_id, b_id) = Quadratic::get_key_contributions(key);
            // let a_vtype = environment.get(&a_id).vtype;
            // let b_vtype = environment.get(&b_id).vtype;

            let v_vtype = environment.get(&var.id).vtype;

            if a_id == var.id || b_id == var.id {
                // We don't care here which other variable is binary, i.e., we don't care if a or b
                // is the matching binary variable. We know that it remains a and b in the
                // quadratic term and the value does not change as the multiplied variable's factor
                // is 1.0.
                match v_vtype {
                    Vtype::Binary => (),
                    Vtype::Spin => (),
                    _ => {
                        let new_higher_order = Some(HigherOrder::new_from_vars_with_value(
                            a_id, b_id, var, *value,
                        ));
                        if higher_order.is_none() {
                            higher_order = new_higher_order;
                        } else {
                            higher_order.as_mut().unwrap().append(new_higher_order);
                        }
                        outvars.remove(key);
                    }
                }
            } else {
                let new_higher_order = Some(HigherOrder::new_from_vars_with_value(
                    a_id, b_id, var, *value,
                ));
                if higher_order.is_none() {
                    higher_order = new_higher_order;
                } else {
                    higher_order.as_mut().unwrap().append(new_higher_order);
                }
                outvars.remove(key);
            }
        }

        (out, higher_order)
    }
}

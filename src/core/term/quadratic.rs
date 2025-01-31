use crate::core::{
    environment::EnvId,
    exceptions::VariablesFromDifferentEnvsError,
    operations::{Term, TermAddition, TermFloatMultiplication, TermSubtraction},
    Environment, VarRef,
};
use std::collections::HashMap;

#[cfg(feature = "py")]
use pyo3::prelude::*;

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

    pub fn new_from_vars(a: &VarRef, b: &VarRef) -> Result<Self, VariablesFromDifferentEnvsError> {
        if a.env_id != b.env_id {
            return Err(VariablesFromDifferentEnvsError);
        }

        let key = Self::make_key(a.id, b.id);
        // println!("key = {}", key);
        let mut variables = HashMap::new();
        variables.insert(key, 1.0);
        Ok(Self {
            env_id: a.env_id,
            variables: Some(variables),
        })
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
                .join(" "),
            None => String::from(""),
        }
    }

    pub fn make_key(a: u32, b: u32) -> u64 {
        // println!("keygen: a = {} and b = {}", a, b);
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

impl Term<u64> for Quadratic {
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
}

impl TermAddition<u64> for Quadratic {}
impl TermSubtraction<u64> for Quadratic {}
impl TermFloatMultiplication<u64> for Quadratic {}

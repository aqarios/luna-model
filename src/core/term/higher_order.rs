use std::{collections::HashMap, ops::AddAssign};

#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::{
    environment::EnvId,
    higher_order_operations::{TermAdditionC, TermC, TermFloatMultiplicationC, TermSubtractionC},
    variable::VarId,
    Environment, VarRef,
};

pub type HigherOrderKey = String;
static DELIMITER: &str = "-";

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq)]
pub struct HigherOrder {
    pub env_id: EnvId,
    variables: Option<HashMap<HigherOrderKey, f64>>,
}

impl HigherOrder {
    pub fn empty(env_id: EnvId) -> Self {
        Self {
            env_id,
            variables: None,
        }
    }

    pub fn new_from_vars_with_value(var_a: u32, var_b: u32, var_c: &VarRef, value: f64) -> Self {
        let mut keys = vec![var_a, var_b, var_c.id];
        let key = Self::make_key(&mut keys);
        let mut variables = HashMap::new();
        variables.insert(key, value);
        Self {
            env_id: var_c.env_id,
            variables: Some(variables),
        }
    }

    pub fn new_from_keys_with_value(
        env_id: EnvId,
        var_a: u32,
        var_b: u32,
        var_c: u32,
        value: f64,
    ) -> Self {
        let mut keys = vec![var_a, var_b, var_c];
        let key = Self::make_key(&mut keys);
        let mut variables = HashMap::new();
        variables.insert(key, value);
        Self {
            env_id,
            variables: Some(variables),
        }
    }

    pub fn new_from_multi_keys_with_value(env_id: EnvId, keys: Vec<VarId>, value: f64) -> Self {
        let mut mutkeys = keys.clone();
        let key = Self::make_key(&mut mutkeys);
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
                    let mut out = String::new();
                    let component = Self::get_key_contributions(key.to_string())
                        .iter()
                        .map(|vid| environment.get(vid).name.clone())
                        .collect::<Vec<String>>()
                        .join(" * ");
                    if *value < 0.0 {
                        out.push_str(&format!("- {} * ", -value));
                    } else if *value == 1.0 {
                        // Nothing happens, we don't show the 1.0
                    } else {
                        out.push_str(&format!("{} * ", value));
                    }
                    out.push_str(&component);
                    out
                })
                .collect::<Vec<String>>()
                .join(" + "),
            None => String::new(),
        }
    }

    pub fn make_key(ids: &mut Vec<VarId>) -> HigherOrderKey {
        ids.sort();
        ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(DELIMITER)
    }

    pub fn update_key(key: HigherOrderKey, new: VarId) -> HigherOrderKey {
        let mut vec = Self::get_key_contributions(key);
        vec.push(new);
        Self::make_key(&mut vec)
    }

    pub fn key_contains_other(key_elements: Vec<VarId>, other: VarId) -> bool {
        key_elements.contains(&other)
    }

    pub fn get_key_contributions(key: HigherOrderKey) -> Vec<VarId> {
        key.split(DELIMITER)
            .map(|s| s.parse::<VarId>().unwrap())
            .collect()
    }

    pub fn add_kv(&mut self, key: HigherOrderKey, value: f64) {
        match self.has_variables() {
            false => {
                let mut nh = HashMap::new();
                nh.insert(key, value);
                self.variables = Some(nh);
            }
            true => {
                let mutvars = self.mutable_variables();
                match mutvars.get_mut(&key) {
                    Some(v) => v.add_assign(value),
                    None => {
                        let _ = mutvars.insert(key, value);
                    }
                }
            }
        }
    }

    pub fn add_elem(&mut self, key_a: u32, key_b: u32, key_c: u32, value: f64) {
        let mut keys = vec![key_a, key_b, key_c];
        let key = Self::make_key(&mut keys);
        self.add_kv(key, value)
    }

    pub fn add_multi(&mut self, keys: Vec<VarId>, value: f64) {
        let mut mutkeys = keys.clone();
        let key = Self::make_key(&mut mutkeys);
        self.add_kv(key, value);
    }

    pub fn set(&mut self, other: &Self) {
        self.variables = other.variables.clone()
    }
}

impl TermC<HigherOrderKey> for HigherOrder {
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

    fn mutable_variables(&mut self) -> &mut HashMap<HigherOrderKey, f64> {
        self.variables.as_mut().unwrap()
    }

    fn variables(&self) -> &HashMap<HigherOrderKey, f64> {
        self.variables.as_ref().unwrap()
    }

    fn fill_variables(
        &mut self,
        variables: HashMap<HigherOrderKey, f64>,
    ) -> &mut HashMap<HigherOrderKey, f64> {
        self.variables.insert(variables)
    }
}

impl TermAdditionC<HigherOrderKey> for HigherOrder {}
impl TermSubtractionC<HigherOrderKey> for HigherOrder {}
impl TermFloatMultiplicationC<HigherOrderKey> for HigherOrder {}

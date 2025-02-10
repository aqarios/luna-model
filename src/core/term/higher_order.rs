use std::collections::HashMap;

#[cfg(feature = "py")]
use pyo3::prelude::*;

use crate::core::{
    environment::EnvId,
    higher_order_operations::{
        TermAdditionC, TermC, TermConstantMultiplicationC, TermFloatMultiplicationC,
        TermMultiplicationC, TermSubtractionC,
    },
    operations::TermLinearMultiplication,
    variable::VarId,
    Environment, VarRef, Vtype,
};

use super::{Linear, Quadratic};

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

    pub fn get_key_contributions(key: HigherOrderKey) -> Vec<VarId> {
        key.split(DELIMITER)
            .map(|s| s.parse::<VarId>().unwrap())
            .collect()
    }

    pub fn append(&mut self, other: Option<HigherOrder>) {
        match other {
            None => (),
            Some(h) => match self.has_variables() {
                true => {
                    let selfvars = self.mutable_variables();
                    for (key, value) in h.variables().iter() {
                        selfvars.insert(key.clone(), *value);
                    }
                }
                false => self.variables = h.variables.clone(),
            },
        }
    }

    pub fn append_elem(&mut self, key_a: u32, key_b: u32, key_c: u32, value: f64) {
        let mut keys = vec![key_a, key_b, key_c];
        let key = Self::make_key(&mut keys);
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
impl TermConstantMultiplicationC<HigherOrderKey> for HigherOrder {}

impl TermMultiplicationC<&VarRef> for HigherOrder {
    fn mul(&self, var: &VarRef, environment: &Environment) -> Self {
        if !self.has_variables() {
            return HigherOrder::empty(self.env_id);
        }
        let mut out = Self::new_from_other(&self);
        let outvars = out.mutable_variables();

        for (key, value) in self.variables().iter() {
            let var_vtype = environment.get(&var.id).vtype;
            let variables = Self::get_key_contributions(key.to_string());

            let mut found_equal: bool = false;
            for varid in variables {
                if varid == var.id {
                    found_equal = true;
                    break;
                }
            }

            if found_equal {
                // Similar to the quadratic case, we don't care which key was matching we only
                // care if any variable contained is Binary or Spin type. If so, we can safely
                // ignore the multiplication with 1.0. In all other cases we register a new
                // higher order entry and remove the old one.
                match var_vtype {
                    Vtype::Binary => (),
                    Vtype::Spin => (),
                    _ => {
                        // We create a new entry with the current varaible and remove the
                        // old one.
                        let new_key = Self::update_key(key.to_string(), var.id);
                        outvars.insert(new_key, *value);
                        outvars.remove(key);
                    }
                }
            } else {
                let new_key = Self::update_key(key.to_string(), var.id);
                outvars.insert(new_key, *value);
                outvars.remove(key);
            }
        }

        out
    }
}

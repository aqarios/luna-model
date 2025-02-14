use crate::core::{
    environment::EnvId,
    exceptions::VariablesFromDifferentEnvsError,
    operations::{Term, TermAddition, TermFloatMultiplication, TermSubtraction},
    variable::VarId,
    Environment, VarRef,
};
use std::{hash::Hash, ops::AddAssign};

#[cfg(feature = "py")]
use pyo3::prelude::*;

use super::{variable_storage::variables_with_capacity, Variables};

// #[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
// pub struct QuadraticKey {
//     value: u64,
// }
// type QuadraticVariables = Variables<QuadraticKey>;

// impl Hash for QuadraticKey {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         state.write_u64(self.value);
//     }
// }

// impl QuadraticKey {
//     fn new(value: u64) -> Self {
//         Self { value }
//     }
// }

pub type QuadraticKey = u64;
type QuadraticVariables = Variables<QuadraticKey>;

pub struct ContainsResult {
    pub not_contained: Option<u32>,
}

impl ContainsResult {
    fn new(not_contained: Option<u32>) -> Self {
        Self { not_contained }
    }
}

pub trait QuadraticKeyContains {
    fn contained(&self, other: VarId) -> Option<ContainsResult>;
}

impl QuadraticKeyContains for QuadraticKey {
    fn contained(&self, other: VarId) -> Option<ContainsResult> {
        let (a, b) = Quadratic::get_key_contributions(self);
        let eq_a = other == a;
        let eq_b = other == b;

        match (eq_a, eq_b) {
            (false, false) => None,
            (true, false) => Some(ContainsResult::new(Some(b))),
            (false, true) => Some(ContainsResult::new(Some(a))),
            // both keys match and both keys are equal.
            // We can return just one. the rest is dependent on the type.
            (true, true) => Some(ContainsResult::new(None)),
        }
    }
}

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq)]
pub struct Quadratic {
    pub env_id: EnvId,
    // variables: Option<QuadraticVariables>,
    variables: QuadraticVariables,
}

impl Quadratic {
    pub fn empty(env_id: EnvId) -> Self {
        Self {
            env_id,
            variables: Variables::default(),
            // variables: None
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
        let mut variables = QuadraticVariables::default();
        variables.insert(key, 1.0);
        Ok(Self {
            env_id: a.env_id,
            // variables: Some(variables),
            variables,
        })
    }

    pub fn new_from_vars_with_value(
        a: &VarRef,
        b: &VarRef,
        value: f64,
    ) -> Result<Self, VariablesFromDifferentEnvsError> {
        if a.env_id != b.env_id {
            return Err(VariablesFromDifferentEnvsError);
        }

        let key = Self::make_key(a.id, b.id);
        let mut variables = QuadraticVariables::default();
        variables.insert(key, value);
        Ok(Self {
            env_id: a.env_id,
            // variables: Some(variables),
            variables,
        })
    }

    pub fn new_from_key_and_var_with_value(a: &u32, b: &VarRef, value: f64) -> Self {
        let key = Self::make_key(*a, b.id);
        let mut variables = QuadraticVariables::default();
        variables.insert(key, value);
        Self {
            env_id: b.env_id,
            // variables: Some(variables),
            variables,
        }
    }

    pub fn new_from_keys_with_value(env_id: EnvId, a: &u32, b: &u32, value: f64) -> Self {
        let key = Self::make_key(*a, *b);
        let mut variables = QuadraticVariables::default();
        variables.insert(key, value);
        Self {
            env_id,
            // variables: Some(variables),
            variables,
        }
    }

    pub fn set(&mut self, other: &Self) {
        self.variables = other.variables.clone()
    }

    pub fn as_string(&self, environment: &Environment) -> String {
        // match &self.variables {
        //   Some(vs) => vs
        self.variables
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
            .join(" + ")
        // None => String::from(""),
        // }
    }

    #[inline]
    pub fn make_key(a: u32, b: u32) -> QuadraticKey {
        // The larger key is always at the end.
        //
        if a < b {
            let au64 = (a as u64) << 32;
            // QuadraticKey::new(au64 | (b as u64))
            au64 | (b as u64)
        } else {
            let bu64 = (b as u64) << 32;
            // QuadraticKey::new(bu64 | (a as u64))
            bu64 | (a as u64)
        }
    }

    pub fn get_key_contributions(key: &QuadraticKey) -> (u32, u32) {
        // ((key.value >> 32) as u32, key.value as u32)
        ((key >> 32) as u32, *key as u32)
    }

    pub fn add_elem(&mut self, key_a: u32, key_b: u32, value: f64) {
        let key = Self::make_key(key_a, key_b);
        self.add_kv(key, value);
    }

    pub fn add_quadratic(&mut self, a: &VarRef, b: &VarRef, value: f64) {
        let key = Self::make_key(a.id, b.id);
        self.add_kv(key, value);
    }

    #[inline]
    pub fn insert_quadratic(&mut self, a: &VarRef, b: &VarRef, value: f64) {
        let key = Self::make_key(a.id, b.id);
        self.mutable_variables().insert(key, value);
    }

    pub fn add_kv(&mut self, key: QuadraticKey, value: f64) {
        match self.has_variables() {
            false => {
                let mut nh = QuadraticVariables::default();
                nh.insert(key, value);
                // self.variables = Some(nh);
                self.variables = nh;
            }
            true => {
                // Check if the key is already contained.
                // If so, we add the new value
                // if not we create it.
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
    pub fn allocate(&mut self, size: usize) {
        match self.has_variables() {
            true => self.mutable_variables().reserve(size),
            false => {
                // let vars = QuadraticVariables::with_capacity(size);
                let vars = variables_with_capacity(size);
                // self.variables = Some(vars);
                self.variables = vars;
            }
        }
    }
}

impl Term<QuadraticKey> for Quadratic {
    fn reset(&mut self) {
        self.variables = Variables::default()
    }

    fn new_from_other(other: &Self) -> Self {
        Self {
            env_id: other.env_id,
            variables: other.variables.clone(),
        }
    }

    fn has_variables(&self) -> bool {
        // self.variables.is_some()
        true
    }

    fn mutable_variables(&mut self) -> &mut QuadraticVariables {
        // self.variables.as_mut().unwrap()
        &mut self.variables
    }

    fn variables(&self) -> &QuadraticVariables {
        // self.variables.as_ref().unwrap()
        &self.variables
    }

    fn fill_variables(&mut self, variables: QuadraticVariables) -> &mut QuadraticVariables {
        // self.variables.insert(variables)
        self.variables = variables;
        &mut self.variables
    }
}

impl TermAddition<QuadraticKey> for Quadratic {}
impl TermSubtraction<QuadraticKey> for Quadratic {}
impl TermFloatMultiplication<QuadraticKey> for Quadratic {}

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

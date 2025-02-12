use crate::core::{
    environment::EnvId,
    exceptions::VariablesFromDifferentEnvsError,
    operations::{Term, TermAddition, TermFloatMultiplication, TermSubtraction},
    variable::VarId,
    Environment, VarRef,
};
use std::{collections::HashMap, ops::AddAssign};

#[cfg(feature = "py")]
use pyo3::prelude::*;

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

    pub fn add_elem(&mut self, key: u32, value: f64) {
        match self.has_variables() {
            false => {
                let mut nh = HashMap::new();
                nh.insert(key, value);
                self.variables = Some(nh);
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

    pub fn set(&mut self, other: &Self) {
        self.variables = other.variables.clone()
    }
}

impl Term<VarId> for Linear {
    fn new_from_other(other: &Self) -> Self {
        match &other.variables {
            Some(v) => Self {
                env_id: other.env_id,
                variables: Some(v.clone()),
            },
            None => Self {
                env_id: other.env_id,
                variables: None,
            },
        }
    }

    fn reset(&mut self) {
        self.variables = None
    }

    fn has_variables(&self) -> bool {
        self.variables.is_some()
    }

    fn mutable_variables(&mut self) -> &mut HashMap<VarId, f64> {
        self.variables.as_mut().unwrap()
    }

    fn variables(&self) -> &HashMap<VarId, f64> {
        self.variables.as_ref().unwrap()
    }

    fn fill_variables(&mut self, variables: HashMap<VarId, f64>) -> &mut HashMap<VarId, f64> {
        self.variables.insert(variables)
    }
}

impl TermAddition<VarId> for Linear {}
impl TermSubtraction<VarId> for Linear {}
impl TermFloatMultiplication<VarId> for Linear {}

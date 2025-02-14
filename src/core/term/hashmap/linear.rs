use crate::core::{
    environment::EnvId,
    exceptions::VariablesFromDifferentEnvsError,
    operations::{Term, TermAddition, TermFloatMultiplication, TermSubtraction},
    variable::VarId,
    Environment, VarRef,
};
use std::ops::AddAssign;

#[cfg(feature = "py")]
use pyo3::prelude::*;

use super::{variable_storage::variables_with_capacity, Variables};

type LinearVariables = Variables<VarId>;

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq)]
pub struct Linear {
    pub env_id: EnvId,
    pub variables: Option<LinearVariables>,
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
        let mut variables = LinearVariables::default();
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

        let mut variables = LinearVariables::default();
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

    pub fn add_linear(&mut self, var: &VarRef, value: f64) {
        self.add_elem(var.id, value);
    }

    #[inline]
    pub fn insert_linear(&mut self, var: &VarRef, value: f64) {
        self.mutable_variables().insert(var.id, value);
    }

    pub fn add_elem(&mut self, key: u32, value: f64) {
        match self.has_variables() {
            false => {
                let mut nh = LinearVariables::default();
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

    pub fn allocate(&mut self, size: usize) {
        match self.has_variables() {
            true => self.mutable_variables().reserve(size),
            false => {
                // let vars = LinearVariables::with_capacity(size);
                let vars = variables_with_capacity(size);
                self.variables = Some(vars);
            }
        }
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

    #[inline]
    fn reset(&mut self) {
        self.variables = None
    }

    #[inline]
    fn has_variables(&self) -> bool {
        self.variables.is_some()
    }

    #[inline]
    fn mutable_variables(&mut self) -> &mut LinearVariables {
        self.variables.as_mut().unwrap()
    }

    #[inline]
    fn variables(&self) -> &LinearVariables {
        self.variables.as_ref().unwrap()
    }

    #[inline]
    fn fill_variables(&mut self, variables: LinearVariables) -> &mut LinearVariables {
        self.variables.insert(variables)
    }
}

impl TermAddition<VarId> for Linear {}
impl TermSubtraction<VarId> for Linear {}
impl TermFloatMultiplication<VarId> for Linear {}

use crate::core::{
    environment::EnvId,
    exceptions::VariablesFromDifferentEnvsError,
    operations::{
        Term, TermAddition, TermFloatMultiplication, TermSubtraction, TermVarMultiplication,
    },
    variable::VarId,
    Environment, VarRef, Vtype,
};
use std::collections::HashMap;

#[cfg(feature = "py")]
use pyo3::prelude::*;

use super::Quadratic;

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

    pub fn append_variable(&mut self, var: &VarRef, value: Option<f64>) {
        match value {
            None => (),
            Some(v) => {
                if v == 0.0 {
                    return;
                }
                match self.has_variables() {
                    true => {
                        let vars = self.mutable_variables();
                        vars.insert(var.id, v);
                    }
                    false => {
                        let mut new = HashMap::new();
                        new.insert(var.id, v);
                        self.variables = Some(new);
                    }
                }
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

    // fn get_vtype(&self, key: &VarId, environment: &Environment) -> Vtype {
    //     // todo: can we remove this clone?
    //     environment.get(key).vtype.clone()
    // }
}

impl TermAddition<VarId> for Linear {}
impl TermSubtraction<VarId> for Linear {}
impl TermFloatMultiplication<VarId> for Linear {}

impl TermVarMultiplication<VarId, Quadratic, u64> for Linear {
    fn mul(&self, rhs: &VarRef, environment: &Environment) -> (Self, Option<Quadratic>) {
        if !self.has_variables() {
            return (Linear::empty(self.env_id), None);
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

        let mut quadratic: Option<Quadratic> = None;

        for (key, value) in self.variables().iter() {
            let cur_vtype = environment.get(key).vtype;

            if *key == rhs.id {
                // The two variables are equal.
                //
                // We need to check the variable types. If Binary or Spin
                // it remains linear after multiplication, else it will
                // be quadratic.
                //
                // We can just look at a single vtype.
                match cur_vtype {
                    Vtype::Binary => (),
                    Vtype::Spin => (),
                    _ => {
                        // creating the new quadratic expression;
                        let new_quadratic =
                            Some(Quadratic::new_from_vars_with_value(key, rhs, *value));

                        if quadratic.is_none() {
                            quadratic = new_quadratic;
                        } else {
                            quadratic.as_mut().unwrap().append(new_quadratic);
                        }
                        // match quadratic {
                        //     None => quadratic = Some(new_quadratic),
                        //     Some(q) => q.append(&new_quadratic),
                        // }
                        // The current key can be removed from the output.
                        outvars.remove(key);
                    }
                }
            } else {
                // The two variables are not equal. Always result in a new
                // quadratic term.
                let new_quadratic = Some(Quadratic::new_from_vars_with_value(key, rhs, *value));
                if quadratic.is_none() {
                    quadratic = new_quadratic;
                } else {
                    quadratic.as_mut().unwrap().append(new_quadratic);
                }
                // match &quadratic {
                //     None => quadratic = Some(new_quadratic),
                //     Some(q) => q.append(&new_quadratic),
                // }
                // The current key can be removed from the output.
                outvars.remove(key);
            }
        }

        (out, quadratic)
    }
}

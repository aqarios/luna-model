use crate::core::expression::One;
use crate::core::traits::ContentEquality;
use crate::core::variable::{VarRef, Variable, Vtype};
use crate::core::writer::LineLengthRestrictor;
use crate::core::{LazyBounds, ValueByIndex, VarAssignment};
use crate::errors::{VariableCreationErr, VariableNotExistingErr};
use crate::types::Bias;
use crate::types::{EnvId, VarIndex};
use derive_more::{Deref, DerefMut};
use global_counter::primitive::exact::CounterU64;
use hashbrown::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::slice::Iter;
use std::{cell::RefCell, ops::Index, rc::Rc};

pub static ENV_COUNTER: CounterU64 = CounterU64::new(0);

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub id: EnvId,
    pub variables: Vec<Variable>,
    pub variables_lookup: HashMap<String, VarIndex>,
    pub varcount: VarIndex,
}

#[derive(Debug, PartialEq, Deref, DerefMut)]
pub struct SharedEnvironment(Rc<RefCell<Environment>>);

impl SharedEnvironment {
    /// Deep clone a shared environment.
    /// This creates a new SharedEnvironment with a deep copy of the contained
    /// environment, not just an increase of the reference counted environment.
    /// The deep cloned environment gets a new environment id that is guaranteed
    /// to be different from all other possibly exisiting environments.
    pub fn deep_clone(&self) -> Self {
        let b = self.borrow();
        let cloned = b.deref().deep_clone();
        SharedEnvironment::new(cloned)
    }

    pub fn add_variable(
        &self,
        name: &str,
        vtype: Option<Vtype>,
        bounds: Option<LazyBounds>,
    ) -> Result<VarRef, VariableCreationErr> {
        let mut mutable_env = self.borrow_mut();
        if mutable_env.variables_lookup.contains_key(name) {
            return Err(VariableCreationErr::VariableExists(name.to_string()));
        }
        ensure_name_valid(name)?;
        let var = Variable::new(name.to_string(), vtype, bounds, mutable_env.id)?;
        let id = mutable_env.varcount;
        mutable_env.variables.push(var);
        mutable_env.variables_lookup.insert(name.to_string(), id);
        mutable_env.varcount += VarIndex::one();
        Ok(VarRef::new(id, self.clone()))
    }

    pub fn add_binary(&self, name: &str) -> Result<VarRef, VariableCreationErr> {
        self.add_variable(name, Some(Vtype::Binary), None)
    }

    pub fn add_spin(&self, name: &str) -> Result<VarRef, VariableCreationErr> {
        self.add_variable(name, Some(Vtype::Spin), None)
    }

    pub fn add_real(
        &self,
        name: &str,
        bounds: Option<LazyBounds>,
    ) -> Result<VarRef, VariableCreationErr> {
        self.add_variable(name, Some(Vtype::Real), bounds)
    }

    pub fn add_integer(
        &self,
        name: &str,
        bounds: Option<LazyBounds>,
    ) -> Result<VarRef, VariableCreationErr> {
        self.add_variable(name, Some(Vtype::Integer), bounds)
    }

    pub fn get_vref_by_name(&self, name: &str) -> Result<VarRef, VariableNotExistingErr> {
        let index = self.borrow().get(&name.to_string())?;
        // As we don't store the VarRefs here, we need to create a new one based on the info
        // we have.
        Ok(VarRef::new(index, self.clone()))
    }

    pub fn get_vrefs_in_order(&self) -> Vec<VarRef> {
        (0..self.borrow().variables.len())
            .map(|idx| VarRef::new(idx.into(), self.clone()))
            .collect()
    }
}

impl SharedEnvironment {
    pub fn new(env: Environment) -> Self {
        Self(Rc::new(RefCell::new(env)))
    }

    pub fn default() -> Self {
        Self(Rc::new(RefCell::new(Environment::new())))
    }
}

impl Clone for SharedEnvironment {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl ContentEquality for SharedEnvironment {
    /// Compare content equality of two environments, ignoring the envid.
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.borrow().is_equal_contents(&other.borrow())
    }
}

impl ContentEquality for Environment {
    /// Compare content equality of two environments, ignoring the envid.
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.variables.is_equal_contents(&other.variables)
            && self.variables_lookup == other.variables_lookup
            && self.varcount == other.varcount
    }
}

impl Environment {
    pub fn new() -> Self {
        Self::new_for(ENV_COUNTER.inc())
    }

    pub fn new_for(id: EnvId) -> Self {
        Self {
            id,
            variables: Vec::new(),
            variables_lookup: HashMap::new(),
            varcount: VarIndex::default(),
        }
    }

    /// Deep clone an environment.
    /// This creates a new Environment with a deep copy of the environment this method
    /// is called on.
    /// The deep cloned environment gets a new environment id that is guaranteed
    /// to be different from all other possibly exisiting environments.
    pub fn deep_clone(&self) -> Self {
        let id = ENV_COUNTER.inc();
        Self {
            id,
            variables: self.variables.iter().map(|v| v.deep_clone(id)).collect(),
            variables_lookup: self.variables_lookup.clone(),
            varcount: self.varcount.clone(),
        }
    }

    /// Alias for self[id].vtype
    #[inline]
    pub fn get_vtype(&self, id: VarIndex) -> Vtype {
        self[id].vtype
    }

    pub fn iter(&self) -> Iter<'_, Variable> {
        self.variables.iter()
    }

    pub fn get(&self, name: &String) -> Result<VarIndex, VariableNotExistingErr> {
        Ok(*(self
            .variables_lookup
            .get(name)
            .ok_or_else(|| VariableNotExistingErr)?))
    }

    pub fn evaluate_bounds<Sample: ValueByIndex<VarIndex, Output = VarAssignment>>(
        &self,
        sample: &Sample,
    ) -> Vec<bool> {
        self.variables
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let value: Bias = sample.value_by_index(i.into()).to_bias();
                v.bounds.evaluate(value)
            })
            .collect()
    }
}

impl Index<VarIndex> for Environment {
    type Output = Variable;

    fn index(&self, index: VarIndex) -> &Self::Output {
        let idx: usize = index.into();
        &self.variables[idx]
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let variables: Vec<_> = self.variables.iter().map(|x| x.name.clone()).collect();
        let mut writer = LineLengthRestrictor::new(0);
        writer
            .write(&format!("Environment {}", self.id))
            .increase_indent()
            .new_line();
        for (i, var) in variables.iter().enumerate() {
            if i > 0 {
                writer.write(",").space();
            }
            writer.write(var);
        }
        f.write_str(&writer.to_string())
    }
}

fn ensure_name_valid(name: &str) -> Result<(), VariableCreationErr> {
    if !name.starts_with(|c: char| c.is_ascii_alphabetic()) {
        Err(VariableCreationErr::VarName(String::from(
            "Variable names must start with an alphabetic character.",
        )))
    } else if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ',')
    {
        Err(VariableCreationErr::VarName(String::from(
            "Variable names must only contain alphanumeric characters or '_' or ','.",
        )))
    } else {
        Ok(())
    }
}

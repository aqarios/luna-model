use crate::core::expression::One;
use crate::core::variable::{VarRef, Variable, Vtype};
use crate::core::writer::LineLengthRestrictor;
use crate::core::{LazyBounds, ValueByIndex, VarAssignment};
use crate::errors::{VariableCreationErr, VariableNotExistingErr};
use crate::types::Bias;
use crate::types::{EnvId, VarIndex};
use derive_more::{Deref, DerefMut};
use global_counter::primitive::exact::CounterU8;
use hashbrown::HashMap;
use std::fmt::{Display, Formatter};
use std::slice::Iter;
use std::{cell::RefCell, ops::Index, rc::Rc};

// already thread safe.
static ENV_COUNTER: CounterU8 = CounterU8::new(0);

#[derive(Debug, PartialEq)]
pub struct Environment {
    pub id: EnvId,
    pub variables: Vec<Variable>,
    pub variables_lookup: HashMap<String, VarIndex>,
    pub varcount: VarIndex,
}

#[derive(Debug, PartialEq, Deref, DerefMut)]
pub struct SharedEnvironment(Rc<RefCell<Environment>>);

impl SharedEnvironment {
    pub fn new(env: Environment) -> Self {
        Self(Rc::new(RefCell::new(env)))
    }
}

impl Clone for SharedEnvironment {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl Environment {
    pub fn new() -> Self {
        Self::new_for(ENV_COUNTER.get())
    }

    pub fn new_for(id: EnvId) -> Self {
        Self {
            id,
            variables: Vec::new(),
            variables_lookup: HashMap::new(),
            varcount: VarIndex::default(),
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

pub fn add_variable(
    env: SharedEnvironment,
    name: &String,
    vtype: Option<&Vtype>,
    bounds: Option<LazyBounds>,
) -> Result<VarRef, VariableCreationErr> {
    let mut mutable_env = env.borrow_mut();
    if mutable_env.variables_lookup.contains_key(name) {
        return Err(VariableCreationErr::VariableExists(name.clone()));
    }
    ensure_name_valid(name)?;
    let var = Variable::new(name.to_string(), vtype, bounds, mutable_env.id)?;
    let id = mutable_env.varcount;
    mutable_env.variables.push(var);
    mutable_env.variables_lookup.insert(name.to_string(), id);
    mutable_env.varcount += VarIndex::one();
    Ok(VarRef::new(id, env.clone()))
}

fn ensure_name_valid(name: &String) -> Result<(), VariableCreationErr> {
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

pub fn get_vref_by_name(
    name: &String,
    env: SharedEnvironment,
) -> Result<VarRef, VariableNotExistingErr> {
    let index = env.borrow().get(name)?;
    // As we don't store the VarRefs here, we need to create a new one based on the info
    // we have.
    Ok(VarRef::new(index, env.clone()))
}

pub fn get_vrefs_in_order(env: SharedEnvironment) -> Vec<VarRef> {
    (0..env.borrow().variables.len())
        .map(|idx| VarRef::new(idx.into(), env.clone()))
        .collect()
}

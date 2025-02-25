use crate::core::{
    exceptions::VariableExistsError,
    expression::IndexConstraints,
    variable::{Bounds, VarRef, Variable, Vtype},
};
use global_counter::primitive::exact::CounterU8;
use hashbrown::HashMap;
use std::{cell::RefCell, ops::Index, rc::Rc};

pub type EnvId = u8;

// already thread safe.
static ENV_COUNTER: CounterU8 = CounterU8::new(0);

#[derive(Debug)]
pub struct Environment<Index> {
    pub id: EnvId,
    pub variables: Vec<Variable>,
    pub variables_lookup: HashMap<String, Index>,
    pub varcount: Index,
}

impl<Index> Environment<Index>
where
    Index: IndexConstraints,
{
    pub fn new() -> Self {
        Self {
            id: ENV_COUNTER.get(),
            variables: Vec::new(),
            variables_lookup: HashMap::new(),
            varcount: Index::default(),
        }
    }

    /// Alias for self[id].vtype
    pub fn get_vtype(&self, id: Index) -> Vtype {
        self[id].vtype
    }
}

impl<Idx> Index<Idx> for Environment<Idx>
where
    Idx: IndexConstraints,
{
    type Output = Variable;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.variables[index.into()]
    }
}

pub fn add_variable<Index: IndexConstraints>(
    env: Rc<RefCell<Environment<Index>>>,
    name: &String,
    vtype: Option<&Vtype>,
    bounds: Option<&Bounds>,
) -> Result<VarRef<Index>, VariableExistsError> {
    let mut mutable_env = env.borrow_mut();
    if mutable_env.variables_lookup.contains_key(name) == true {
        return Err(VariableExistsError);
    }

    let var = Variable::new(name.to_string(), vtype, bounds, mutable_env.id);
    let id = mutable_env.varcount;
    mutable_env.variables.push(var);
    mutable_env.variables_lookup.insert(name.to_string(), id);
    mutable_env.varcount += Index::one();
    Ok(VarRef::new(id, env.clone()))
}

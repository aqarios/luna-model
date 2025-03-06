use crate::core::utils::LineLengthRestrictor;
use crate::core::{
    exceptions::VariableExistsError,
    expression::IndexConstraints,
    variable::{Bounds, VarRef, Variable, Vtype},
};
use global_counter::primitive::exact::CounterU8;
use hashbrown::HashMap;
use std::fmt::{Display, Formatter};
use std::slice::Iter;
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
    #[inline]
    pub fn get_vtype(&self, id: Index) -> Vtype {
        self[id].vtype
    }

    pub fn iter(&self) -> Iter<'_, Variable> {
        self.variables.iter()
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

impl<Index> Display for Environment<Index>
where
    Index: IndexConstraints,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let variables: Vec<_> = self.variables.iter().map(|x| x.name.clone()).collect();
        let mut writer = LineLengthRestrictor::new(0);
        writer.write(&format!("Environment {}", self.id));
        writer.increase_indent();
        writer.new_line();
        for (i, var) in variables.iter().enumerate() {
            if i > 0 {
                writer.write(",");
                writer.space();
            }
            writer.write(var);
        }
        f.write_str(&writer.to_string())
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

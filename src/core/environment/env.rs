use super::storage::VariableStorage;
use crate::core::{
    exceptions::VariableExistsError,
    variable::{Bounds, VarId, VarRef, Variable, Vtype},
};
use global_counter::primitive::exact::CounterU8;
use std::collections::HashMap;

#[cfg(feature = "py")]
use pyo3::prelude::*;

pub type EnvId = u8;

static ENV_COUNTER: CounterU8 = CounterU8::new(0);

#[cfg_attr(feature = "py", pyclass)]
pub struct Environment {
    pub id: EnvId,
    pub variables: VariableStorage,
    pub variables_lookup: HashMap<String, VarId>,
    pub varcount: u32,
    // u32 should be by far enough information for all vars (4_294_967_295)
}

impl Environment {
    pub fn new() -> Self {
        Self {
            id: ENV_COUNTER.get(),
            variables: VariableStorage::new(),
            variables_lookup: HashMap::new(),
            varcount: 0,
        }
    }
}

impl Environment {
    // todo: add remaning parameters
    pub fn add_var(
        &mut self,
        name: &String,
        vtype: Option<Vtype>,
        bounds: Option<Bounds>,
    ) -> Result<VarRef, VariableExistsError> {
        if self.variables_lookup.contains_key(name) == true {
            return Err(VariableExistsError);
        }

        self.varcount += 1;
        // println!("adding variable '{}' with key '{}'", name, self.varcount);
        let var = Variable::new(name.to_string(), vtype, bounds, self.id);
        let varref = VarRef::new(self.varcount, self.id);
        self.variables.push(var);
        self.variables_lookup.insert(name.to_string(), varref.id);
        Ok(varref)
    }

    pub fn get(&self, key: &VarId) -> &Variable {
        // println!("getting variable for key: '{}'", key);
        self.variables.get((key - 1) as usize).unwrap()
    }
}

#[cfg(feature = "py")]
#[pymethods]
impl Environment {
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(Environment::new())
    }
}

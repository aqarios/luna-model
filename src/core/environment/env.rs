use crate::core::{
    exceptions::VariableExistsError,
    variable::{Bounds, VarId, VarRef, Variable, Vtype},
};
use global_counter::primitive::exact::CounterU8;
use std::{cell::RefCell, collections::HashMap, ops::Index, rc::Rc, usize};

// #[cfg(feature = "py")]
// use pyo3::prelude::*;

pub type EnvId = u8;

static ENV_COUNTER: CounterU8 = CounterU8::new(0);

// #[cfg_attr(feature = "py", pyclass)]
pub struct Environment {
    pub id: EnvId,
    pub variables: Vec<Variable>,
    pub variables_lookup: HashMap<String, VarId>,
    pub varcount: u32,
    // u32 should be by far enough information for all vars (4_294_967_295)
}

impl Environment {
    pub fn new() -> Self {
        Self {
            id: ENV_COUNTER.get(),
            variables: Vec::new(),
            variables_lookup: HashMap::new(),
            varcount: 0,
        }
    }
}

impl Environment {
    pub fn drop_var(&mut self, var_id: VarId) {
        let variable = &self.variables[var_id.0 as usize];
        self.variables_lookup.remove(&variable.name);
        self.variables.remove(var_id.0 as usize);
        self.varcount -= 1;
    }
}

// impl Index<VarId> for Vec<Variable> {
//     type Output = Variable;
//
//     fn index(&self, index: VarId) -> &Self::Output {
//         &self[index.0 as usize]
//     }
// }

pub fn add_varibale(
    env: Rc<RefCell<Environment>>,
    name: &String,
    vtype: Option<&Vtype>,
    bounds: Option<&Bounds>,
) -> Result<VarRef, VariableExistsError> {
    let mut mutable_env = env.borrow_mut();

    if mutable_env.variables_lookup.contains_key(name) == true {
        return Err(VariableExistsError);
    }

    let var = Variable::new(name.to_string(), vtype, bounds, mutable_env.id);
    let varid = mutable_env.varcount;
    mutable_env.variables.push(var);
    mutable_env
        .variables_lookup
        .insert(name.to_string(), VarId(varid));
    mutable_env.varcount += 1;

    Ok(VarRef::new(varid, env.clone()))
}

// impl Environment {
//     // todo: add remaning parameters
//     pub fn add_var(
//         &mut self,
//         name: &String,
//         vtype: Option<&Vtype>,
//         bounds: Option<&Bounds>,
//     ) -> Result<VarRef, VariableExistsError> {
//         if self.variables_lookup.contains_key(name) == true {
//             return Err(VariableExistsError);
//         }
//
//         // println!("adding variable '{}' with key '{}'", name, self.varcount);
//         let var = Variable::new(name.to_string(), vtype, bounds, self.id);
//         let varref = VarRef::new(self.varcount, self.id);
//         self.variables.push(var);
//         self.variables_lookup.insert(name.to_string(), varref.id);
//         self.varcount += 1;
//         Ok(varref)
//     }
//
//     // pub fn get(&self, var_id: &VarId) -> &Variable {
//     //     // println!("getting variable for key: '{}'", key);
//     //     let key: usize = *var_id.into();
//     //     self.variables.get(key.into()).unwrap()
//     // }
// }

// #[cfg(feature = "py")]
// #[pymethods]
// impl Environment {
//     #[new]
//     fn py_new() -> PyResult<Self> {
//         Ok(Environment::new())
//     }
// }

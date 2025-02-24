use crate::core::{
    exceptions::VariableExistsError,
    expression::IndexConstraints,
    variable::{Bounds, VarRef, Variable, Vtype},
};
use global_counter::primitive::exact::CounterU8;
use hashbrown::HashMap;
use std::{cell::RefCell, ops::Index, rc::Rc};

pub type EnvId = u8;

static ENV_COUNTER: CounterU8 = CounterU8::new(0);

/// Wrapper around the Variable to indicate a dead behaviour
/// for convenience and readability.
// pub type DeadableVariable = Option<Variable>;
// #[derive(Debug)]
// pub struct DeadableVariable {
//     var: Option<Variable>,
// }
//
// impl DeadableVariable {
//     // fn new(name: String, vtype: Option<&Vtype>, bounds: Option<&Bounds>, env_id: EnvId) -> Self {
//     //     Self {
//     //         var: Some(Variable::new(name, vtype, bounds, env_id)),
//     //     }
//     // }
//
//     fn new_from_var(var: Variable) -> Self {
//         Self { var: Some(var) }
//     }
//
//     fn dead() -> Self {
//         Self { var: None }
//     }
//
//     fn is_dead(&self) -> bool {
//         self.var.is_none()
//     }
//
//     /// This fails if the variable is not dead...
//     /// should always check deadness before
//     fn get(&self) -> &Variable {
//         println!("self = {:?}", self);
//         self.var.as_ref().unwrap()
//     }
//
//     /// This fails if the variable is not dead...
//     /// should always check deadness before
//     fn get_maybe(&self) -> &Option<Variable> {
//         println!("self = {:?}", self);
//         &self.var
//     }
// }

#[derive(Debug)]
pub struct Environment<Index> {
    pub id: EnvId,
    pub variables: Vec<Variable>,
    pub variables_lookup: HashMap<String, Index>,
    pub varcount: Index,
    // u32 should be by far enough information for all vars (4_294_967_295)
    // Stuff to track variables for deletion logic.
    // pub deadcount: u32,
    // pub largest_dead: u32,
    // pub deadvars: Vec<VarId>,
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
            // deadvars: Vec::new(),
            // deadcount: 0,
            // largest_dead: 0,
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

// impl Environment {
//     pub fn delete_var(&mut self, var_id: VarId) {
//         // println!("drop_var(var_id = {})", var_id);
//         // println!(
//         //     "drop_var(var_id = {}) self.variables.len() = {}",
//         //     var_id,
//         //     self.variables.len()
//         // );
//         let variable = &self.variables[var_id.0 as usize];
//         // println!("variable = {:?}", variable);
//         assert!(variable.is_some(), "cannot delete the same variable twice");
//
//         self.variables_lookup
//             .remove(&variable.as_ref().unwrap().name);
//
//         // Let's assume the following variables:
//         //
//         // [u, v, w, x, y, z]
//         //
//         // If we delete the first variable (u) we have:
//         //
//         // [dead, v, w, x, y, z]
//         //
//         // If we delete the last variable (z) we have:
//         //
//         // [dead, v, w, x, y]
//         //
//         // Now it get's a bit more tricky
//         //
//         // If we delete variable x we have
//         //
//         // [dead, v, w, dead, y]
//         //
//         // If we now delete the last variable (y) we have:
//         //
//         // [dead, v, w, dead, dead]
//         //
//         //    Which should be turned into
//         //
//         // [dead, v, w]
//         //
//         //    immediately
//         //
//         // Thus we need to track the last living variable index.
//         // In this case this would be index of variable y before the operation
//         // and variable w after the delte operation.
//         //
//         // This logic needs to be implemented efficiently.
//         // For now we just track the number of dead variables.
//         // if the number of dead variables is equal to the total number of
//         // variables ever registered we can safely replace the vector dropping
//         // the allocated memory.
//         //
//         // We also need removal on expressions and how to handle this.
//         // Probably easiest is to have an error when a variable is delted
//         // that is contained in an expression.
//
//         // Check if removing the variable would create a dead variable.
//         // I.e., if the removed variable is the last variable, i.e., the index
//         // is equal to the length - 1. We can completely remove it.
//         // else we need to set the value to the dead value to ensure correct indexing.
//         if var_id.0 as usize == self.variables.len() - 1 {
//             self.variables.remove(var_id.0 as usize);
//         } else {
//             // We need to set the value to indicate a dead variable.
//             self.variables[var_id.0 as usize] = DeadableVariable::default();
//             // We register the variable as a dead variable.
//             self.deadvars.push(var_id);
//         }
//         // Reduce the variable count by one.
//         self.varcount -= 1;
//         // Check if all variables are dead. If so free the memory.
//         if self.deadvars.len() == self.variables.len() {
//             self.variables = Vec::new();
//             self.deadvars = Vec::new();
//         }
//
//         // println!(
//         //     "variables = {:?}",
//         //     self.variables
//         //         .iter()
//         //         .map(|v| v.as_ref().and_then(|e| Some(e.name.clone())))
//         //         .map(|v| v.unwrap_or(String::from("dead")))
//         //         .collect::<Vec<String>>()
//         // );
//     }
//
//     pub fn clear(&mut self) {
//         // Does not clear memory, keeps allocations.
//         self.variables_lookup.clear();
//         self.variables.clear();
//     }
// }

// impl Index<VarId> for Vec<Variable> {
//     type Output = Variable;
//
//     fn index(&self, index: VarId) -> &Self::Output {
//         &self[index.0 as usize]
//     }
// }

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

    // Check if dead variables are in the current environment.
    // If so, we set the varid to the first element from the dead variables.
    // Then we retract one from the deadvars and remove the dead index from the
    // deadvars vector.
    // let varid: u32;
    // if mutable_env.deadcount != 0 {
    //     varid = *mutable_env.deadvars.last().unwrap();
    //     mutable_env.variables[varid as usize] = var;
    // } else {
    //     // Else we set the varid to the current varcount and
    //     // increase the variables by one.
    //     varid = mutable_env.varcount;
    //     mutable_env.variables.push(var);
    // }

    let id = mutable_env.varcount;
    mutable_env.variables.push(var);
    mutable_env.variables_lookup.insert(name.to_string(), id);
    mutable_env.varcount += Index::one();
    Ok(VarRef::new(id, env.clone()))
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

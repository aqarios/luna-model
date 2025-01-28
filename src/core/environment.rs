use super::{storage::VariableStorage, varref::VarRef, Variable};

#[cfg(feature = "py")]
use pyo3::prelude::*;

#[cfg_attr(feature = "py", pyclass)]
pub struct Environment {
    pub variables: VariableStorage,
    // pub variables: Vec<Variable>,
    pub varcount: u32, // should be enough information for all vars
                       // maybe need additional metadata
}

impl Environment {
    pub fn new() -> Self {
        Self {
            // variables: VariableStorage::empty(),
            variables: VariableStorage::new(),
            varcount: 0,
        }
    }

    // todo: other params AND CHECK IF A VARIABLE WITH THIS NAME IS ALREADY CONTAINED, IF SO THROW
    // ERROR
    pub fn add_var(&mut self, name: String) -> VarRef {
        let var = Variable::new(name);
        let varref = VarRef::new(self.varcount);
        self.variables.push(var);
        self.varcount += 1;
        varref
    }

    pub fn get_var(&self, id: u32) -> &Variable {
        // todo: error handling
        self.variables.get(id as usize).unwrap()
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

use global_counter::primitive::exact::CounterU64;
use hashbrown::HashMap;

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{EnvIdx, VarIdx, Vtype};

use super::util::ensure_name_valid;
use crate::{
    environment::util::{ensure_unused, freeidx},
    variable::{LazyBounds, VarRef, Variable},
};

pub static ENV_COUNTER: CounterU64 = CounterU64::new(0);

#[derive(Debug)]
pub struct Environment {
    id: EnvIdx,
    variables: HashMap<VarIdx, Variable>,
    lookup: HashMap<String, VarIdx>,
    inverted: Vec<VarIdx>,
    freeidx: Vec<VarIdx>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            id: ENV_COUNTER.inc(),
            variables: HashMap::new(),
            lookup: HashMap::new(),
            inverted: Vec::new(),
            freeidx: Vec::new(),
        }
    }

    pub fn insert<'a>(
        &'a mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> LunaModelResult<VarRef<'a>> {
        ensure_name_valid(name)?;
        ensure_unused(&self.lookup, name)?;
        let var = Variable::new(name, vtype, bounds, self.id)?;
        let idx = freeidx(&mut self.freeidx, self.variables.len() as VarIdx);
        self.variables.insert(idx, var);
        self.lookup.insert(name.into(), idx);
        Ok(VarRef::new(idx, self))
    }

    pub fn insert_inverted<'a>(&'a mut self, base: &VarRef) -> LunaModelResult<VarRef<'a>> {
        let nvars = self.variables.len() as VarIdx;
        let basevar = self.variables.get_mut(&base.id).unwrap();
        if basevar.vtype != Vtype::Binary {
            return Err(LunaModelError::InvalidInversion(
                format!("vtype {} cannot be inverted", basevar.vtype).into(),
            ));
        }
        let inv_name = basevar.name.inverted();
        ensure_unused(&self.lookup, &inv_name)?;
        let idx = freeidx(&mut self.freeidx, nvars);
        let mut var = Variable::new(&inv_name, Vtype::InvertedBinary, None, self.id)?;

        var.inverted = Some(base.id);
        basevar.inverted = Some(idx);

        self.variables.insert(idx, var);
        self.lookup.insert(inv_name.into(), idx);
        Ok(VarRef::new(idx, self))
    }

    pub fn remove(&mut self, target: &VarRef) {
        let name = &self.variables[&target.id].name;
        self.lookup.remove(&name.0);
        self.variables.remove(&target.id);
        self.freeidx.push(target.id);
    }
}

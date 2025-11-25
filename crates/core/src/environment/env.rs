use std::ops::Index;

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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Environment {
    pub(crate) id: EnvIdx,
    pub(crate) variables: HashMap<VarIdx, Variable>,
    pub(crate) lookup: HashMap<String, VarIdx>,
    pub(crate) inverted: Vec<VarIdx>,
    pub(crate) freeidx: Vec<VarIdx>,
}

impl Environment {
    pub fn default() -> Self {
        Self {
            id: ENV_COUNTER.inc(),
            variables: HashMap::new(),
            lookup: HashMap::new(),
            inverted: Vec::new(),
            freeidx: Vec::new(),
        }
    }

    pub fn insert(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> LunaModelResult<VarIdx> {
        ensure_name_valid(name)?;
        ensure_unused(&self.lookup, name)?;
        let var = Variable::new(name, vtype, bounds, self.id)?;
        let idx = freeidx(&mut self.freeidx, self.variables.len() as VarIdx);
        self.variables.insert(idx, var);
        self.lookup.insert(name.into(), idx);
        // Ok(VarRef::new(idx, Arc::new(RwLock::new(self))))
        Ok(idx)
    }

    pub fn insert_inverted(&mut self, base: &VarRef) -> LunaModelResult<VarIdx> {
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
        // Ok(VarRef::new(idx, self))
        Ok(idx)
    }

    pub fn remove(&mut self, target: &VarRef) {
        let name = &self.variables[&target.id].name;
        self.lookup.remove(&name.0);
        self.variables.remove(&target.id);
        self.freeidx.push(target.id);
    }
}

impl Index<VarIdx> for Environment {
    type Output = Variable;

    fn index(&self, index: VarIdx) -> &Self::Output {
        self.variables
            .get(&index)
            .expect(&format!("no variable for index '{index}'"))
    }
}

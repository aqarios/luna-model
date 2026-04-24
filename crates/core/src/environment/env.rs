//! Concrete mutable environment implementation.

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use global_counter::primitive::exact::CounterU64;
use indexmap::IndexMap;
use sqids::Sqids;
use std::collections::HashMap;

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{EnvIdx, VarIdx, Vtype};

use super::util::ensure_name_valid;
use crate::{
    bounds::LazyBounds,
    environment::util::ensure_unused,
    variable::{VarRef, Variable},
};

pub static ENV_COUNTER: CounterU64 = CounterU64::new(0);

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub(crate) id: EnvIdx,
    pub(crate) variables: IndexMap<VarIdx, Variable>,
    pub(crate) lookup: HashMap<String, VarIdx>,
    pub(crate) next_idx: VarIdx,
}

impl Default for Environment {
    /// Creates an empty environment with a fresh unique identifier.
    fn default() -> Self {
        Self {
            id: ENV_COUNTER.inc(),
            variables: IndexMap::new(),
            lookup: HashMap::new(),
            next_idx: 0,
        }
    }
}

impl Environment {
    /// Creates a new environment from pre-populated variable storage.
    ///
    /// A fresh environment id is always assigned even when the rest of the
    /// content is provided by the caller. That behavior is intentional: the id
    /// models environment identity rather than serialized content.
    pub fn new(
        variables: IndexMap<VarIdx, Variable>,
        lookup: HashMap<String, VarIdx>,
        next_idx: VarIdx,
    ) -> Self {
        Self {
            id: ENV_COUNTER.inc(),
            variables,
            lookup,
            next_idx,
        }
    }

    /// Clones the environment contents into a new environment identity.
    pub fn deep_clone(&self) -> Self {
        let id = ENV_COUNTER.inc();
        Self {
            id,
            variables: self.variables.clone(),
            lookup: self.lookup.clone(),
            next_idx: self.next_idx,
        }
    }

    /// Returns the unique environment id.
    pub fn id(&self) -> usize {
        self.id as usize
    }

    /// Returns the next variable index that will be assigned on insertion.
    pub fn next_idx(&self) -> VarIdx {
        self.next_idx
    }

    /// Returns the number of variables stored in the environment.
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// Returns `true` when the environment contains no variables.
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }

    /// Iterates over the stored variable indices in insertion order.
    pub fn vars(&self) -> impl Iterator<Item = VarIdx> {
        self.variables.keys().copied()
    }

    /// Inserts a new variable and returns its assigned index.
    ///
    /// This validates the name, ensures the name is unused, concretizes lazy
    /// bounds against the variable type, and then updates both the index map and
    /// the name lookup table.
    pub fn insert(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> LunaModelResult<VarIdx> {
        ensure_name_valid(name)?;
        ensure_unused(&self.lookup, name)?;
        let var = Variable::new(name, vtype, bounds)?; //, self.id)?;
        let idx = self.next_idx;
        self.variables.insert(idx, var);
        self.lookup.insert(name.into(), idx);
        self.next_idx += 1;
        Ok(idx)
    }

    /// Inserts a variable, falling back to a derived unique name on collisions.
    ///
    /// The fallback path is primarily used by importers and translators that
    /// must preserve progress even when an external format contains duplicate or
    /// inconvenient names.
    pub fn insert_with_fallback(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
        enc: Option<&[u64]>,
    ) -> LunaModelResult<VarIdx> {
        let ret = self.insert(name, vtype, bounds);

        match ret {
            Err(LunaModelError::VariableExists(_)) => {
                let content = match enc {
                    Some(e) => e,
                    // unwrap here is safe as variable exists.
                    Option::None => &[(*self.lookup.get(name).unwrap()).into()],
                };
                let suffix = Sqids::default().encode(content).unwrap();
                let new_name = format!("{}_{}", name, suffix);
                self.insert(&new_name, vtype, bounds)
            }
            _ => ret,
        }
    }

    /// Creates the inverted companion variable for a binary variable.
    ///
    /// The environment links both variables through their `inverted` fields so
    /// later transformations can move between them without searching by name.
    pub fn insert_inverted(&mut self, base: &VarRef) -> LunaModelResult<VarIdx> {
        let basevar = self.variables.get_mut(&base.id).unwrap();
        if basevar.vtype != Vtype::Binary {
            return Err(LunaModelError::InvalidInversion(
                format!("vtype {} cannot be inverted", basevar.vtype).into(),
            ));
        }
        let inv_name = basevar.name.inverted();
        ensure_unused(&self.lookup, &inv_name)?;
        let idx = self.next_idx;
        let mut var = Variable::new(&inv_name, Vtype::InvertedBinary, None)?; //, self.id)?;

        var.inverted = Some(base.id);
        basevar.inverted = Some(idx);

        self.variables.insert(idx, var);
        self.lookup.insert(inv_name.into(), idx);
        self.next_idx += 1;
        Ok(idx)
    }

    /// Removes a variable by reference from both storage maps.
    pub fn remove(&mut self, target: &VarRef) {
        let name = &self.variables[&target.id].name;
        self.lookup.remove(&name.0);
        self.variables.shift_remove(&target.id);
    }

    /// Looks up a variable index by name.
    pub fn lookup(&self, name: &str) -> LunaModelResult<VarIdx> {
        self.lookup
            .get(name)
            .ok_or_else(|| LunaModelError::VariableNotExisting(name.into()))
            .copied()
    }

    /// Returns an immutable variable by index.
    pub fn get(&self, index: VarIdx) -> LunaModelResult<&Variable> {
        self.variables
            .get(&index)
            .ok_or_else(|| LunaModelError::VariableNotExisting(index.to_string().into()))
    }

    /// Returns a mutable variable by index.
    pub fn get_mut(&mut self, index: VarIdx) -> LunaModelResult<&mut Variable> {
        self.variables
            .get_mut(&index)
            .ok_or_else(|| LunaModelError::VariableNotExisting(index.to_string().into()))
    }

    /// Returns the variable type for a variable identified by name.
    pub fn vtype_of(&self, name: &str) -> LunaModelResult<Vtype> {
        Ok(self
            .variables
            .get(
                self.lookup
                    .get(name)
                    .ok_or(LunaModelError::VariableNotExisting(name.to_string().into()))?,
            )
            .ok_or(LunaModelError::VariableNotExisting(name.to_string().into()))?
            .vtype)
    }
}

impl Index<VarIdx> for Environment {
    type Output = Variable;

    /// Indexes directly by variable index and panics on missing variables.
    fn index(&self, index: VarIdx) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl IndexMut<VarIdx> for Environment {
    /// Mutably indexes by variable index and panics on missing variables.
    fn index_mut(&mut self, index: VarIdx) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        _ = f;
        unimplemented!()
    }
}

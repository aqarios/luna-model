//! Shared, lock-backed environment wrapper.

use derive_more::Deref;
use parking_lot::RwLock;
use std::sync::Arc;

use super::Environment;
use crate::{bounds::LazyBounds, traits::ContentEquality, variable::VarRef};

use lunamodel_error::LunaModelResult;
use lunamodel_types::{EnvIdx, VarIdx, Vtype};

#[derive(Debug, Clone, Default, Deref)]
pub struct ArcEnv {
    /// Shared lock-protected environment storage.
    pub env: Arc<RwLock<Environment>>,
}

impl From<Arc<RwLock<Environment>>> for ArcEnv {
    /// Wraps an existing shared environment handle.
    fn from(env: Arc<RwLock<Environment>>) -> Self {
        Self { env }
    }
}

impl From<Environment> for ArcEnv {
    /// Wraps an owned environment in `Arc<RwLock<_>>`.
    fn from(env: Environment) -> Self {
        Self {
            env: Arc::new(RwLock::new(env)),
        }
    }
}

impl ArcEnv {
    /// Deep-clones the underlying environment into a fresh shared wrapper.
    pub fn deep_clone(&self) -> Self {
        self.env.read_arc().deep_clone().into()
    }

    /// Returns the number of variables in the environment.
    pub fn len(&self) -> usize {
        self.env.read_arc().len()
    }

    /// Returns `true` when the environment contains no variables.
    pub fn is_empty(&self) -> bool {
        self.env.read_arc().is_empty()
    }

    /// Returns all variables as environment-bound references.
    ///
    /// This allocates a `Vec` because each returned [`VarRef`] must capture the
    /// shared environment wrapper. The method favors simplicity over a custom
    /// iterator type.
    pub fn vars(&self) -> Vec<VarRef> {
        // maybe we can return iter when passing closure for vref gen.
        self.env
            .read_arc()
            .vars()
            .map(|v| VarRef::new(v, self.clone()))
            .collect()
    }

    /// Inserts a new variable and returns a reference tied to this environment.
    pub fn insert(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> LunaModelResult<VarRef> {
        Ok(VarRef::new(
            self.env.write_arc().insert(name, vtype, bounds)?,
            self.clone(),
        ))
    }

    /// Inserts a variable, deriving a fallback name when necessary.
    pub fn insert_with_fallback(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
        enc: Option<&[u64]>,
    ) -> LunaModelResult<VarRef> {
        Ok(VarRef::new(
            self.env
                .write_arc()
                .insert_with_fallback(name, vtype, bounds, enc)?,
            self.clone(),
        ))
    }

    /// Inserts the inverted companion of a binary variable.
    pub fn insert_inverted(&mut self, base: &VarRef) -> LunaModelResult<VarRef> {
        Ok(VarRef::new(
            self.env.write_arc().insert_inverted(base)?,
            self.clone(),
        ))
    }

    /// Removes the referenced variable from the environment.
    pub fn remove(&mut self, target: &VarRef) {
        self.env.write_arc().remove(target);
    }

    /// Returns the unique id of the wrapped environment.
    pub fn id(&self) -> EnvIdx {
        self.env.read_arc().id
    }

    /// Looks up a variable by name and returns a bound variable reference.
    pub fn lookup(&self, name: &str) -> LunaModelResult<VarRef> {
        Ok(VarRef::new(self.env.read_arc().lookup(name)?, self.clone()))
    }

    /// Returns a variable reference for a known index.
    ///
    /// This does not validate the index immediately. Any later metadata access
    /// through the returned `VarRef` will fail if the index does not exist.
    pub fn get(&self, idx: VarIdx) -> VarRef {
        VarRef::new(idx, self.clone())
    }

    /// Returns whether a variable with the given name exists.
    pub fn contains(&self, name: &str) -> bool {
        self.env.read_arc().lookup.contains_key(name)
    }

    /// Returns the variable type for a named variable.
    pub fn vtype_of(&self, name: &str) -> LunaModelResult<Vtype> {
        self.env.read_arc().vtype_of(name)
    }

    /// Sorts variable names according to their environment insertion order.
    ///
    /// This is mainly useful when external sources provide variable names in an
    /// arbitrary order but later code wants deterministic output aligned with
    /// the model's internal ordering.
    pub fn sort(&self, mut vars: Vec<String>) -> Vec<String> {
        let env = &self.env.read_arc();
        vars.sort_by_key(|a| env.lookup(a).unwrap());
        vars
    }
}

impl ContentEquality for ArcEnv {
    /// Compares environments by variable contents rather than shared pointer identity.
    fn equal_contents(&self, other: &Self) -> bool {
        for v in self.vars() {
            let vname = v.name().unwrap();
            match other.lookup(&vname) {
                Err(_) => return false,
                Ok(o) => {
                    if v.vtype().unwrap() != o.vtype().unwrap() {
                        return false;
                    }
                    if v.bounds().unwrap() != o.bounds().unwrap() {
                        return false;
                    }
                }
            }
        }
        true
    }
}

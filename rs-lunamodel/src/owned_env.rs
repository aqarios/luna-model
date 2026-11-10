//! Owned-environment facade backed by the host's shared environment handle.

use std::{collections::HashMap, ops::Deref};

use indexmap::IndexMap;

use crate::{Env, LazyBounds, Result, TryDeepClone, VarRef, Variable, Vtype, traits::ToPtrList};

/// Mutable variable snapshot returned by [`Environment::get_mut`].
///
/// LunaModel's only publicly mutable `Variable` field is `inverted`. The proxy
/// exposes that field directly and commits it to the host when dropped. Call
/// [`commit`](Self::commit) when an ABI error must be observed.
#[must_use = "changes are committed when the proxy is dropped; call commit() to observe errors"]
pub struct VariableMut<'a> {
    env: &'a mut Env,
    index: u32,
    variable: Variable,
    pub inverted: Option<u32>,
    committed: bool,
}

impl VariableMut<'_> {
    pub fn commit(mut self) -> Result<()> {
        self.commit_inner()?;
        self.committed = true;
        Ok(())
    }

    fn commit_inner(&mut self) -> Result<()> {
        let (present, inverted) = self.inverted.map_or((false, 0), |value| (true, value));
        check!(env_set_inverted(@mut self.env, self.index, present, inverted))
    }
}

impl Deref for VariableMut<'_> {
    type Target = Variable;
    fn deref(&self) -> &Self::Target {
        &self.variable
    }
}

impl Drop for VariableMut<'_> {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.commit_inner();
        }
    }
}

/// Concrete environment facade matching `lunamodel_core::Environment`.
///
/// The storage remains in the host process; unlike [`Env`] (the `ArcEnv`
/// facade), methods return raw variable indices and owned variable snapshots.
#[derive(Default)]
pub struct Environment {
    inner: Env,
}

impl Environment {
    pub fn new(
        variables: IndexMap<u32, Variable>,
        lookup: HashMap<String, u32>,
        next_idx: u32,
    ) -> Result<Self> {
        let variable_ptrs = variables.values().map(Variable::as_ptr).collect::<Vec<_>>();
        let indices = variables.keys().copied().collect::<Vec<_>>();
        let (lookup_names, lookup_indices): (Vec<_>, Vec<_>) = lookup.into_iter().unzip();
        let lookup_ptrs = lookup_names.as_slice().to_ptr_list()?;
        let inner: Env = out!(env_new(
            variable_ptrs.as_ptr(),
            indices.as_ptr(),
            indices.len(),
            lookup_ptrs.as_ptr(),
            lookup_indices.as_ptr(),
            lookup_indices.len(),
            next_idx
        ))?;
        Ok(Self { inner })
    }

    pub fn deep_clone(&self) -> Result<Self> {
        Ok(Self {
            inner: self.inner.try_deep_clone(())?,
        })
    }

    pub fn id(&self) -> Result<u128> {
        self.inner.id()
    }
    pub fn next_idx(&self) -> Result<u32> {
        self.inner.next_idx()
    }
    pub fn len(&self) -> Result<usize> {
        self.inner.len()
    }
    pub fn is_empty(&self) -> Result<bool> {
        self.inner.is_empty()
    }

    pub fn vars(&self) -> Result<std::vec::IntoIter<u32>> {
        Ok(self
            .inner
            .vars()?
            .into_iter()
            .map(|var| var.id())
            .collect::<Result<Vec<_>>>()?
            .into_iter())
    }

    pub fn insert(&mut self, name: &str, vtype: Vtype, bounds: Option<LazyBounds>) -> Result<u32> {
        self.inner.insert(name, vtype, bounds)?.id()
    }

    pub fn insert_with_fallback(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
        enc: Option<&[u64]>,
    ) -> Result<u32> {
        self.inner
            .insert_with_fallback(name, vtype, bounds, enc)?
            .id()
    }

    pub fn insert_inverted(&mut self, base: &VarRef) -> Result<u32> {
        self.inner.insert_inverted(base)?.id()
    }

    pub fn remove(&mut self, target: &VarRef) -> Result<()> {
        self.inner.remove(target)
    }
    pub fn lookup(&self, name: &str) -> Result<u32> {
        self.inner.lookup(name)?.id()
    }
    pub fn get(&self, index: u32) -> Result<Variable> {
        self.inner.get_variable(index)
    }

    pub fn get_mut(&mut self, index: u32) -> Result<VariableMut<'_>> {
        let variable = self.inner.get_variable(index)?;
        let inverted = variable.inverted()?;
        Ok(VariableMut {
            env: &mut self.inner,
            index,
            variable,
            inverted,
            committed: false,
        })
    }

    pub fn vtype_of(&self, name: &str) -> Result<Vtype> {
        self.inner.vtype_of(name)
    }

    pub fn as_arc(&self) -> &Env {
        &self.inner
    }
    pub fn into_arc(self) -> Env {
        self.inner
    }
}

impl From<Environment> for Env {
    fn from(value: Environment) -> Self {
        value.inner
    }
}

impl From<Env> for Environment {
    fn from(inner: Env) -> Self {
        Self { inner }
    }
}

impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        out_or!(env_eq(@self.inner, @other.inner), false)
    }
}

impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Environment")
            .field("id", &self.id().ok())
            .field("next_idx", &self.next_idx().ok())
            .finish()
    }
}

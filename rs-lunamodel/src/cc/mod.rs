//! Owning wrapper over a LunaModel cc handle.
//!
//! Mirrors the Rust `ConstraintCollection`, dispatching every call through the installed
//! [`LunaModelCApi`](lunamodel_cext_vtable::LunaModelCApi) table. Fallible ABI
//! calls surface as [`Result`]; the C ABI can always report a status, so methods
//! that are infallible on the native `ConstraintCollection` still return `Result` here.

use lunamodel_cext_vtable::LmCC;
use lunamodel_types::{Comparator, Vtype};
use rs_lunamodel_derive::RawHandle;
use std::ops::{Deref, DerefMut};

use crate::{
    Constraint, ContentEquality, Env, Expression, Result, Serialization, TryClone, TryDeepClone,
    TryIndex, VarRef, api::api,
};

/// Owning handle to a LunaModel cc (frees on drop).
#[derive(RawHandle)]
pub struct ConstraintCollection {
    handle: *mut LmCC,
}

/// Editable snapshot returned by [`ConstraintCollection::iter_mut`].
#[must_use = "mutations are committed when the editor is dropped; call commit() to observe errors"]
pub struct ConstraintCollectionEditor<'a> {
    target: &'a mut ConstraintCollection,
    items: Vec<(String, Constraint)>,
    committed: bool,
}

impl ConstraintCollectionEditor<'_> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Constraint)> {
        self.items.iter_mut().map(cc_item_mut)
    }

    pub fn commit(mut self) -> Result<()> {
        self.commit_inner()?;
        self.committed = true;
        Ok(())
    }

    fn commit_inner(&mut self) -> Result<()> {
        for (name, constraint) in &self.items {
            self.target.set_constraint(name, constraint)?;
        }
        Ok(())
    }
}

fn cc_item_mut(item: &mut (String, Constraint)) -> (&String, &mut Constraint) {
    (&item.0, &mut item.1)
}

impl Deref for ConstraintCollectionEditor<'_> {
    type Target = [(String, Constraint)];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for ConstraintCollectionEditor<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<'editor> IntoIterator for &'editor mut ConstraintCollectionEditor<'_> {
    type Item = (&'editor String, &'editor mut Constraint);
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'editor, (String, Constraint)>,
        fn(&'editor mut (String, Constraint)) -> (&'editor String, &'editor mut Constraint),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut().map(cc_item_mut)
    }
}

impl Drop for ConstraintCollectionEditor<'_> {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.commit_inner();
        }
    }
}

impl Default for ConstraintCollection {
    fn default() -> Self {
        Self::from_raw((api().cc_default)())
    }
}

impl ConstraintCollection {
    pub fn new() -> Self {
        Self::from_raw((api().cc_new)())
    }

    pub fn vtypes(&self) -> Result<Vec<Vtype>> {
        out!(cc_vtypes(@self))
    }

    pub fn ctypes(&self) -> Result<Vec<Comparator>> {
        out!(cc_ctypes(@self))
    }

    pub fn vars(&self) -> Result<Vec<VarRef>> {
        out!(cc_vars(@self))
    }

    pub fn complete_vars(&self) -> Result<Vec<VarRef>> {
        out!(cc_complete_vars(@self))
    }

    pub fn is_empty(&self) -> Result<bool> {
        out!(cc_is_empty(@self))
    }

    pub fn len(&self) -> Result<usize> {
        out!(cc_len(@self))
    }

    pub fn get(&self, key: &str) -> Result<Constraint> {
        out!(cc_get(@self, !key))
    }

    pub fn contains(&self, key: &str) -> Result<bool> {
        out!(cc_contains(@self, !key))
    }

    pub fn names(&self) -> Result<Vec<String>> {
        out!(cc_names(@self))
    }

    pub fn iter(&self) -> Result<std::vec::IntoIter<(String, Constraint)>> {
        let values = self
            .names()?
            .into_iter()
            .map(|name| Ok((name.clone(), self.get(&name)?)))
            .collect::<Result<Vec<_>>>()?;
        Ok(values.into_iter())
    }

    pub fn iter_mut(&mut self) -> Result<ConstraintCollectionEditor<'_>> {
        let items = self.iter()?.collect();
        Ok(ConstraintCollectionEditor {
            target: self,
            items,
            committed: false,
        })
    }

    pub fn add_many_constraints<I>(&mut self, constraints: I) -> Result<Vec<String>>
    where
        I: IntoIterator<Item = (Constraint, Option<String>)>,
    {
        constraints
            .into_iter()
            .map(|(constraint, name)| self.add_constraint(&constraint, name.as_deref()))
            .collect()
    }

    pub fn evaluate_sample<S>(
        &self,
        sample: &S,
        tol: Option<f64>,
    ) -> Result<std::collections::HashMap<String, bool>>
    where
        for<'a> S: TryIndex<&'a str, Output = f64>,
    {
        self.iter()?
            .map(|(name, constraint)| Ok((name, constraint.evaluate_sample(sample, tol)?)))
            .collect()
    }

    pub fn evaluate_sample_quick(
        &self,
        lookup: &[f64],
        tol: Option<f64>,
    ) -> Result<std::collections::HashMap<String, bool>> {
        self.iter()?
            .map(|(name, constraint)| Ok((name, constraint.evaluate_sample_quick(lookup, tol)?)))
            .collect()
    }

    pub fn add_constraint(&mut self, constr: &Constraint, name: Option<&str>) -> Result<String> {
        out!(cc_add_constraint(@mut self, @constr, ?!name))
    }

    pub fn add_collection(&mut self, other: &Self, prefix: Option<&str>) -> Result<Vec<String>> {
        out!(cc_add_collection(@mut self, @other, ?!prefix))
    }

    pub fn set_constraint(&mut self, key: &str, constr: &Constraint) -> Result<()> {
        check!(cc_set_constraint(@mut self, !key, @constr))
    }

    pub fn remove_constraint(&mut self, key: &str) -> Result<()> {
        check!(cc_remove_constraint(@mut self, !key))
    }

    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> Result<()> {
        check!(cc_substitute(@mut self, @target, @replacement))
    }
}

impl TryClone for ConstraintCollection {
    fn try_clone(&self) -> Result<Self> {
        out!(cc_clone(@self))
    }
}

impl<'a> TryDeepClone<'a> for ConstraintCollection {
    type Target = &'a Env;

    fn try_deep_clone(&self, env: Self::Target) -> Result<Self> {
        out!(cc_deep_clone(@self, @env))
    }
}

impl PartialEq for ConstraintCollection {
    fn eq(&self, other: &Self) -> bool {
        out_or!(cc_eq(@self, @other), false)
    }
}

impl ContentEquality for ConstraintCollection {
    fn equal_contents(&self, other: &Self) -> Result<bool> {
        out!(cc_equal_contents(@self, @other))
    }
}

impl Serialization<'_> for ConstraintCollection {
    type Payload = Env;

    fn serialize(&self) -> Result<Vec<u8>> {
        out!(cc_serialize(@self))
    }

    fn deserialize(data: &[u8], env: Self::Payload) -> Result<Self> {
        out!(cc_deserialize(data.as_ptr(), data.len(), @env))
    }
}

impl TryIndex<&str> for ConstraintCollection {
    type Output = Constraint;
    fn try_index(&self, index: &str) -> Result<Self::Output> {
        self.get(index)
    }
}

impl IntoIterator for ConstraintCollection {
    type Item = (String, Constraint);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
            .expect("failed to iterate a host-backed ConstraintCollection")
    }
}

impl TryFrom<indexmap::IndexMap<String, Constraint>> for ConstraintCollection {
    type Error = crate::Error;

    fn try_from(value: indexmap::IndexMap<String, Constraint>) -> Result<Self> {
        let mut collection = Self::new();
        for (name, constraint) in value {
            collection.add_constraint(&constraint, Some(&name))?;
        }
        Ok(collection)
    }
}

impl std::fmt::Debug for ConstraintCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstraintCollection")
            .field("names", &self.names().ok())
            .finish()
    }
}

impl Drop for ConstraintCollection {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            free!(self, cc_free)
        }
    }
}

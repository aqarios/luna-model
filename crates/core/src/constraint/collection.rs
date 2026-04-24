use crate::{Expression, environment::ArcEnv, traits::ContentEquality, variable::VarRef};
use indexmap::IndexMap;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Comparator, Vtype};
use lunamodel_utils::{unique, unique_by};
use std::ops::Index;

use super::constr::Constraint;

/// Insertion-ordered collection of named constraints.
///
/// The collection uses [`IndexMap`] so iteration order is stable and matches the
/// order in which constraints were inserted. That matters for reproducible
/// serialization, translation, and developer-facing debugging.
#[derive(Default, Debug, Clone)]
pub struct ConstraintCollection {
    /// A map to help in indexing into this collection when [ConstraintKey].
    /// Supports both [ConstraintKey::Str] and [ConstraintKey::Int] but [ConstraintKey::Int] is
    /// not reliable as the order might change when constraints are removed or readded.
    /// [ConstraintKey::Int] will be deprecated going forward.
    data: IndexMap<String, Constraint>,
}

impl ConstraintCollection {
    /// Creates an empty constraint collection.
    pub fn new() -> Self {
        Self {
            data: IndexMap::new(),
        }
    }
    /// Clones the collection into a different shared environment.
    ///
    /// This is separate from ordinary `Clone` because constraints ultimately
    /// refer back to environment-owned variables through their expressions.
    /// `deep_clone` preserves the algebraic contents while re-rooting the whole
    /// collection into `env`.
    pub fn deep_clone(&self, env: ArcEnv) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|(k, c)| (k.clone(), c.deep_clone(env.clone())))
                .collect(),
        }
    }

    /// Returns the distinct variable types referenced by the collection.
    pub fn vtypes(&self) -> impl Iterator<Item = Vtype> {
        unique(self.data.iter().flat_map(|(_, c)| c.lhs.vtypes()))
    }

    /// Returns the distinct comparator types used by the contained constraints.
    pub fn ctypes(&self) -> impl Iterator<Item = Comparator> {
        unique(self.data.iter().map(|(_, c)| c.comparator))
    }

    /// Returns the distinct variables referenced by all left-hand side expressions.
    pub fn vars(&self) -> impl Iterator<Item = VarRef> {
        unique_by(self.data.iter().flat_map(|(_, c)| c.lhs.vars()), |v| v.id)
    }

    /// Iterates over `(name, constraint)` pairs in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Constraint)> {
        self.data.iter()
    }

    /// Iterates mutably over `(name, constraint)` pairs in insertion order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Constraint)> {
        self.data.iter_mut()
    }

    /// Returns `true` when the collection contains no constraints.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of constraints in the collection.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Inserts a constraint, optionally overriding its current name.
    ///
    /// If `name` is `None`, the collection either keeps the constraint's
    /// existing explicit name or synthesizes a fallback name for auto-named
    /// constraints. The inserted constraint is renamed to the final collection
    /// key so the two stay in sync.
    pub fn add_constraint(
        &mut self,
        mut constr: Constraint,
        name: Option<String>,
    ) -> LunaModelResult<String> {
        let fallback = match constr.auto_name {
            true => format!("c{}", self.len()),
            false => constr.name().to_string(),
        };
        let name = name.unwrap_or(fallback);
        if self.data.contains_key(&name) {
            Err(LunaModelError::DuplicateConstraintName(name.into()))
        } else {
            constr.set_name(name.clone())?;
            self.data.insert(name.clone(), constr);
            Ok(name)
        }
    }

    /// Inserts many constraints and returns the final assigned names.
    pub fn add_many_constraints(
        &mut self,
        other: impl Iterator<Item = (Constraint, Option<String>)>,
    ) -> LunaModelResult<Vec<String>> {
        other
            .map(|(constr, name)| self.add_constraint(constr, name))
            .collect::<LunaModelResult<_>>()
    }

    /// Inserts another collection, optionally prefixing all incoming names.
    pub fn add_collection(
        &mut self,
        other: ConstraintCollection,
        prefix: Option<String>,
    ) -> LunaModelResult<Vec<String>> {
        other
            .into_iter()
            .map(|(mut name, constr)| {
                if let Some(p) = prefix.as_ref() {
                    name = format!("{}{}", p, name)
                }
                self.add_constraint(constr, Some(name))
            })
            .collect::<LunaModelResult<_>>()
    }

    /// Replaces an existing constraint while keeping its collection key stable.
    pub fn set_constraint(&mut self, key: &str, constr: Constraint) -> LunaModelResult<()> {
        if let Some(c) = self.data.get_mut(key) {
            *c = constr;
            Ok(())
        } else {
            Err(LunaModelError::NoConstraintForKey(key.to_string().into()))
        }
    }

    /// Removes a constraint by name.
    pub fn remove_constraint(&mut self, key: &str) -> LunaModelResult<()> {
        if self.data.contains_key(key) {
            _ = self.data.shift_remove(key);
            Ok(())
        } else {
            Err(LunaModelError::NoConstraintForKey(key.to_string().into()))
        }
    }

    /// A [VarRef] in the LHS [Expression] of all [Constraint]s in the [ConstraintCollection] collection can be
    /// replaced by a new [Expression] using this function.
    /// This is a convenience function to enable the [substitution operation](Constraint::substitute)
    /// on the LHS expression of all constraints within the constraints collection.
    /// All substitution logic and operations are defined on the [Expression]
    /// [here](Expression::substitute).
    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> LunaModelResult<()> {
        for (_, constr) in self.data.iter_mut() {
            constr.substitute(target, replacement)?;
        }
        Ok(())
    }

    /// Retrieves a constraint by name.
    pub fn get(&self, key: &str) -> LunaModelResult<&Constraint> {
        self.data
            .get(key)
            .ok_or_else(|| LunaModelError::NoConstraintForKey(key.to_string().into()))
    }
}

impl ContentEquality for ConstraintCollection {
    /// Compares semantic contents while ignoring environment identity.
    fn equal_contents(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }
        for (name, constr) in &self.data {
            match other.data.get(name) {
                Some(other_constr) => {
                    if !constr.equal_contents(other_constr) {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
    }
}

impl From<IndexMap<String, Constraint>> for ConstraintCollection {
    /// Wraps an existing `IndexMap` as a constraint collection.
    fn from(data: IndexMap<String, Constraint>) -> Self {
        Self { data }
    }
}

impl IntoIterator for ConstraintCollection {
    type Item = (String, Constraint);
    type IntoIter = indexmap::map::IntoIter<String, Constraint>;

    /// Consumes the collection and yields owned `(name, constraint)` pairs.
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Index<&str> for ConstraintCollection {
    type Output = Constraint;

    /// Indexes directly by constraint name and panics if the key is missing.
    ///
    /// Prefer [`ConstraintCollection::get`] when absence is part of normal control flow.
    fn index(&self, index: &str) -> &Self::Output {
        self.data.get(index).unwrap()
    }
}

impl PartialEq for ConstraintCollection {
    /// Compares constraints including their identity-sensitive equality behavior.
    fn eq(&self, other: &Self) -> bool {
        for (cname, constr) in self.data.iter() {
            if let Ok(otr_constr) = other.get(cname) {
                if constr != otr_constr {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

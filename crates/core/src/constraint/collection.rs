use crate::{Expression, environment::ArcEnv, traits::ContentEquality, variable::VarRef};
use indexmap::IndexMap;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Comparator, Vtype};
use lunamodel_utils::{unique, unique_by};
use std::ops::Index;

use super::constraint::Constraint;

/// The ConstraintCollection struct is an insertion ordered collection of one or more [Constraint]s.
#[derive(Default, Debug, Clone)]
pub struct ConstraintCollection {
    /// A map to help in indexing into this collection when [ConstraintKey].
    /// Supports both [ConstraintKey::Str] and [ConstraintKey::Int] but [ConstraintKey::Int] is
    /// not reliable as the order might change when constraints are removed or readded.
    /// [ConstraintKey::Int] will be deprecated going forward.
    data: IndexMap<String, Constraint>,
}

impl ConstraintCollection {
    pub fn new() -> Self {
        Self {
            data: IndexMap::new(),
        }
    }
    /// Since the [ConstraintCollection] collection (indirectly via the [Constraint]s' LHS [Expression]s) have a reference to
    /// a [SharedEnvironment] we cannot simply check by equality using the builin (derived) equality
    /// primitives. Since two [Constraint]s might be equal albeit being defined in different
    /// [SharedEnvironment]s.
    /// We still want to maintain the option to check if two [Constraint]s are identical (including the
    /// [SharedEnvironment]) so we define this function to keep the `==` operator for identity checks.
    pub fn deep_clone(&self, env: ArcEnv) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|(k, c)| (k.clone(), c.deep_clone(env.clone())))
                .collect(),
        }
    }

    pub fn vtypes(&self) -> impl Iterator<Item = Vtype> {
        unique(self.data.iter().map(|(_, c)| c.lhs.vtypes()).flatten())
    }

    pub fn ctypes(&self) -> impl Iterator<Item = Comparator> {
        unique(self.data.iter().map(|(_, c)| c.comparator))
    }

    pub fn vars(&self) -> impl Iterator<Item = VarRef> {
        unique_by(self.data.iter().map(|(_, c)| c.lhs.vars()).flatten(), |v| {
            v.id
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Constraint)> {
        self.data.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = (String, Constraint)> {
        self.data.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Constraint)> {
        self.data.iter_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn add_constraint(
        &mut self,
        mut constr: Constraint,
        name: Option<String>,
    ) -> LunaModelResult<()> {
        let fallback = match constr.auto_name {
            true => format!("c{}", self.len()),
            false => constr.name().to_string(),
        };
        let name = name.unwrap_or_else(|| fallback);
        if self.data.contains_key(&name) {
            Err(LunaModelError::DuplicateConstraintName(name.into()))
        } else {
            constr.set_name(name.clone())?;
            self.data.insert(name, constr);
            Ok(())
        }
    }

    pub fn add_many_constraints(
        &mut self,
        other: impl Iterator<Item = (Constraint, Option<String>)>,
    ) -> LunaModelResult<()> {
        for (constr, name) in other {
            self.add_constraint(constr, name)?
        }
        Ok(())
    }

    pub fn add_collection(
        &mut self,
        other: ConstraintCollection,
        prefix: Option<String>,
    ) -> LunaModelResult<()> {
        for (mut name, constr) in other.into_iter() {
            match prefix.as_ref() {
                Some(p) => name = format!("{}{}", p, name),
                None => (),
            }
            self.add_constraint(constr, Some(name))?
        }
        Ok(())
    }

    pub fn set_constraint(&mut self, key: &str, constr: Constraint) -> LunaModelResult<()> {
        if let Some(c) = self.data.get_mut(key) {
            *c = constr;
            Ok(())
        } else {
            Err(LunaModelError::NoConstraintForKey(key.to_string().into()))
        }
    }

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

    pub fn get(&self, key: &str) -> LunaModelResult<&Constraint> {
        self.data
            .get(key)
            .ok_or_else(|| LunaModelError::NoConstraintForKey(key.to_string().into()))
    }
}

impl ContentEquality for ConstraintCollection {
    fn equal_contents(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }
        for (name, constr) in &self.data {
            match other.data.get(name) {
                Some(other_constr) => {
                    if !constr.equal_contents(&other_constr) {
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
    fn from(data: IndexMap<String, Constraint>) -> Self {
        Self { data }
    }
}

impl Index<&str> for ConstraintCollection {
    type Output = Constraint;

    fn index(&self, index: &str) -> &Self::Output {
        self.data.get(index).unwrap()
    }
}

impl PartialEq for ConstraintCollection {
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

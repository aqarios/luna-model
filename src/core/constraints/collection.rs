use indexmap::IndexMap;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;

use crate::core::writer::ModelWriter;
use crate::core::{
    Comparator, Constraint, ContentEquality, Expression, ExpressionBase, SharedEnvironment, VarRef,
    Vtype,
};
use crate::errors::{
    DifferentEnvsErr, DuplicateConstraintNameErr, GetConstraintErr, IndexOutOfBoundsErr,
};

#[cfg(feature = "py")]
use pyo3::prelude::*;

/// A [Constraint] can be either identified by an Int or a String. Access is unified by this enum.
///
/// Note: This is subject to change in the future to allow indexing only using a constraints name
/// (String) to ensure a more consistent and user safe API. In addition, it is required to enable
/// enhancements in the transformation stack for operations working on constraints. For more
/// details see <https://github.com/aqarios/luna-model/issues/400>.
#[cfg_attr(feature = "py", derive(FromPyObject))]
pub enum ConstraintKey {
    // #[deprecated(note = "This ")]
    Int(usize),
    Str(String),
}

impl Display for ConstraintKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(idx) => write!(f, "{}", idx),
            Self::Str(name) => write!(f, "{}", name),
        }
    }
}

/// The ConstraintCollection struct is an insertion ordered collection of one or more [Constraint]s.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintCollection {
    // /// A map to help in indexing into this collection when [ConstraintKey::Str] is used.
    pub data: IndexMap<String, Constraint>,
    // /// All [Constraint]s contained in the collection in the order they were added to [Self].
    // pub constraints: Vec<Constraint>,
}

impl ConstraintCollection {
    /// Create an empty ConstraintCollection collection.
    pub fn default() -> Self {
        Self {
            data: IndexMap::new(),
        }
    }

    /// Create a ConstraintCollection collection from another constraints collection. The newly created
    /// object has the same reference to the [SharedEnvironment] as the constraints it is created
    /// from. I.e., it does not use the [Constraint::deep_clone] for all constraints of the given
    /// constraints collection.
    pub fn new_from(other: &Self) -> Self {
        Self {
            data: other.data.clone(),
        }
    }

    /// Create a ConstraintCollection collection from multiple [Constraint] instances contained in a `Vec<_>`.
    pub fn new_from_vec(constraints: Vec<Constraint>) -> Self {
        let mut slf = Self::default();
        constraints
            .into_iter()
            .enumerate()
            .for_each(|(idx, mut c)| {
                if c.has_placeholder_name() {
                    c.set_name_for_idx(idx)
                };
                slf.data.insert(c.name.clone(), c);
            });
        slf
    }

    /// Since the [ConstraintCollection] collection (indirectly via the [Constraint]s' LHS [Expression]s) have a reference to
    /// a [SharedEnvironment] we cannot simply check by equality using the builin (derived) equality
    /// primitives. Since two [Constraint]s might be equal albeit being defined in different
    /// [SharedEnvironment]s.
    /// We still want to maintain the option to check if two [Constraint]s are identical (including the
    /// [SharedEnvironment]) so we define this function to keep the `==` operator for identity checks.
    pub fn deep_clone(&self, env: SharedEnvironment) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|(k, c)| (k.clone(), c.deep_clone(env.clone())))
                .collect(),
        }
    }

    /// Get the number of [Constraint]s contained in the constraints collection.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Iterate over all [Constraint]s in the constraint collection.
    pub fn iter(&self) -> ConstraintCollectionIterator<'_> {
        ConstraintCollectionIterator::new(&self)
    }

    /// Check if the [ConstraintCollection] collection contains any data.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a specific [Constraint] from the constraints collection given a [ConstraintKey].
    /// Returns an error if no constraint exists at the index if [ConstraintKey::Int]` is
    /// passed or if no constraint for the name exists if [ConstraintKey::Str] is passed.
    pub fn get_constraint(&self, key: ConstraintKey) -> Result<&Constraint, GetConstraintErr> {
        match &key {
            ConstraintKey::Int(idx) => {
                if let Some((_, constr)) = self.data.get_index(*idx) {
                    Ok(constr)
                } else {
                    Err(GetConstraintErr::IndexOutOfBoundsErr(
                        IndexOutOfBoundsErr::new(*idx, self.len()),
                    ))
                }
            }
            ConstraintKey::Str(name) => {
                if let Some(constr) = self.data.get(name) {
                    Ok(constr)
                } else {
                    Err(GetConstraintErr::NoConstraintForKeyErr(key.to_string()))
                }
            }
        }
    }

    /// Set (overwrite) a new [Constraint] for the given [ConstraintKey]. A constraint for this key
    /// must already exist. If no constraint is available for the given [ConstraintKey] an error is
    /// returned.
    pub fn set_constraint(
        &mut self,
        key: ConstraintKey,
        constr: Constraint,
    ) -> Result<(), GetConstraintErr> {
        match &key {
            ConstraintKey::Int(idx) => {
                if let Some((_, c)) = self.data.get_index_mut(*idx) {
                    *c = constr;
                    Ok(())
                } else {
                    Err(GetConstraintErr::IndexOutOfBoundsErr(
                        IndexOutOfBoundsErr::new(*idx, self.len()),
                    ))
                }
            }
            ConstraintKey::Str(name) => {
                if let Some(c) = self.data.get_mut(name) {
                    *c = constr;
                    Ok(())
                } else {
                    Err(GetConstraintErr::NoConstraintForKeyErr(key.to_string()))
                }
            }
        }
    }

    /// Remove a [Constraint] for the given [ConstraintKey]. If no cosntraint exists for the given
    /// [ConstraintKey], this function is esentially a no-op.
    pub fn remove_constraint(&mut self, key: ConstraintKey) {
        match &key {
            ConstraintKey::Int(idx) => {
                let name_key = if let Some((name, _)) = self.data.get_index(*idx) {
                    Some(name.clone())
                } else {
                    // no-op
                    Option::None
                };
                if let Some(nk) = name_key {
                    _ = self.data.swap_remove(&nk)
                };
            }
            ConstraintKey::Str(name) => {
                if self.data.contains_key(name) {
                    _ = self.data.swap_remove(name);
                }
            }
        }
    }

    /// A [VarRef] in the LHS [Expression] of all [Constraint]s in the [ConstraintCollection] collection can be
    /// replaced by a new [Expression] using this function.
    /// This is a convenience function to enable the [substitution operation](Constraint::substitute)
    /// on the LHS expression of all constraints within the constraints collection.
    /// All substitution logic and operations are defined on the [Expression]
    /// [here](Expression::substitute).
    pub fn substitute(
        &mut self,
        target: &VarRef,
        replacement: &Expression,
    ) -> Result<(), DifferentEnvsErr> {
        for (_, constr) in self.data.iter_mut() {
            constr.substitute(target, replacement)?;
        }
        Ok(())
    }

    /// In contrast to what this function might suggest based on it's name it has nothing to do
    /// with `C` (language) types. This function returns all **unique** [Comparator]s used by all
    /// [Constraint]s within this [ConstraintCollection] collection.
    pub fn ctypes(&self) -> Vec<Comparator> {
        self.data
            .iter()
            .map(|(_, c)| c.comparator)
            .unique()
            .collect_vec()
    }

    /// This function returns all **unique** [Variable types](Vtype) used by the LHS of all
    /// [Constraint]s within this [ConstraintCollection] collection.
    pub fn vtypes(&self) -> Vec<Vtype> {
        self.data
            .iter()
            .map(|(_, c)| c.lhs.vtypes())
            .flatten()
            .unique()
            .collect_vec()
    }

    /// Implements mutable adding a [Constraint] to the [ConstraintCollection] collection.
    /// This function does not implement the `+=` operation, and this function needs to be
    /// called to mutably add a [Constraint] to [Self].
    pub fn add_assign(&mut self, rhs: &Constraint) -> Result<(), DuplicateConstraintNameErr> {
        if rhs.has_placeholder_name() {
            let mut cloned = rhs.clone();
            cloned.set_name_for_idx(self.len());
            self.data.insert(cloned.name.to_string(), cloned);
            Ok(())
        } else {
            if self.data.contains_key(&rhs.name) {
                Err(DuplicateConstraintNameErr(rhs.name.to_string()))
            } else {
                self.data.insert(rhs.name.to_string(), rhs.clone());
                Ok(())
            }
        }
    }
}

/// Implements adding a [Constraint] to the [ConstraintCollection] collection for a copy using the
/// `+` operation.
impl Add<Constraint> for ConstraintCollection {
    /// Addition can result in either a new [ConstraintCollection] instance or a [DuplicateConstraintNameErr].
    /// See [Self::add] for details.
    type Output = Result<ConstraintCollection, DuplicateConstraintNameErr>;
    /// Implements adding a [Constraint] to the [ConstraintCollection] collection using the `+` operation on
    /// [ConstraintCollection]. This operation might return an error in case the name of the constraint to
    /// be added is already registered within the target constraints collection ([DuplicateConstraintNameErr]).
    /// A new [ConstraintCollection] object is returned from this function based on `self`. This function does
    /// not edit `self`!
    fn add(self, rhs: Constraint) -> Self::Output {
        let mut out = ConstraintCollection::new_from(&self);
        out.add_assign(&rhs)?;
        Ok(out)
    }
}

impl Add<&Constraint> for &ConstraintCollection {
    type Output = Result<ConstraintCollection, DuplicateConstraintNameErr>;

    /// Borrowed version of [ConstraintCollection::add].
    fn add(self, rhs: &Constraint) -> Self::Output {
        let mut out = ConstraintCollection::new_from(&self);
        out.add_assign(rhs)?;
        Ok(out)
    }
}

impl Display for ConstraintCollection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = ModelWriter::new().write_constraints(&self).to_string();
        f.write_str(&s)
    }
}

impl ContentEquality for ConstraintCollection {
    /// Check if two [ConstraintCollection] have equal contents. Uses [Constraint::is_equal_contents] for
    /// all [Constraint]s. See [here](Constraint::is_equal_contents) for details.
    fn is_equal_contents(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }
        for (name, constr) in &self.data {
            if !other.data.contains_key(name) {
                return false;
            }
            if !constr.is_equal_contents(other.data.get(name).unwrap()) {
                return false;
            }
        }
        true
    }
}

pub struct ConstraintCollectionIterator<'a> {
    collection: &'a ConstraintCollection,
    current: usize,
}

impl<'a> ConstraintCollectionIterator<'a> {
    fn new(collection: &'a ConstraintCollection) -> Self {
        Self {
            collection,
            current: 0,
        }
    }
}

impl<'a> Iterator for ConstraintCollectionIterator<'a> {
    type Item = (&'a String, &'a Constraint);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.collection.len() {
            return None;
        }
        let (name, constr) = self.collection.data.get_index(self.current).unwrap();
        self.current += 1;
        Some((name, constr))
    }
}

use crate::{environment::ArcEnv, traits::ContentEquality, variable::VarRef};
use indexmap::IndexMap;
use lunamodel_types::Vtype;
use lunamodel_utils::{unique, unique_by};
use std::{
    fmt::{Display, Formatter},
    ops::Index,
};

use super::constraint::Constraint;

/// A [Constraint] can be either identified by an Int or a String. Access is unified by this enum.
///
/// Note: This is subject to change in the future to allow indexing only using a constraints name
/// (String) to ensure a more consistent and user safe API. In addition, it is required to enable
/// enhancements in the transformation stack for operations working on constraints. For more
/// details see <https://github.com/aqarios/aq-models-rs/issues/400>.
pub enum ConstraintKey {
    /// Will be deprecated going forward.
    Int(usize),
    /// The only viable method to access constraints going forward.
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
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConstraintCollection {
    /// A map to help in indexing into this collection when [ConstraintKey].
    /// Supports both [ConstraintKey::Str] and [ConstraintKey::Int] but [ConstraintKey::Int] is
    /// not reliable as the order might change when constraints are removed or readded.
    /// [ConstraintKey::Int] will be deprecated going forward.
    data: IndexMap<String, Constraint>,
}

impl ConstraintCollection {
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

    pub fn vars(&self) -> impl Iterator<Item = VarRef> {
        unique_by(self.data.iter().map(|(_, c)| c.lhs.vars()).flatten(), |v| {
            v.id
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Constraint)> {
        self.data.iter()
    }
}

impl ContentEquality for ConstraintCollection {
    fn is_equal_contents(&self, other: &Self) -> bool {
        _ = other;
        unimplemented!()
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

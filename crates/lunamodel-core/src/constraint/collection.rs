use indexmap::IndexMap;
use std::fmt::{Display, Formatter};
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
#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintCollection {
    /// A map to help in indexing into this collection when [ConstraintKey].
    /// Supports both [ConstraintKey::Str] and [ConstraintKey::Int] but [ConstraintKey::Int] is 
    /// not reliable as the order might change when constraints are removed or readded.
    /// [ConstraintKey::Int] will be deprecated going forward.
    pub data: IndexMap<String, Constraint>,
}

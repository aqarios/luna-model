use lunamodel_types::{Bias, Comparator};

use crate::{ArcEnv, expression::Expression};

/// [Constraint] names must have a name after <https://github.com/aqarios/aq-models-rs/issues/400>.
/// We need a default name that does not clash with anything. Let's choose an illegal name.
/// A [Constraint] cannot be encoded on it's own. So we can safely assume this name never makes it
/// into a format where this name is illegal. Let's choose "<NN>" since it matches the previous
/// default value exactly.
pub const DEFAULT_CONSTRAINT_NAME: &str = "<NN>";

/// [Constraint] names that can fail when translating to other formats due to bugs in
/// their LP file reading, interpreting any words that start with the constants elements
/// as a number instead of a string. Thus, we need to disallow them as well to ensure
/// consistency and elevated developer experience. The failing readers do not provide a
/// good error message so we catch theses cases early and show the users and appropriate
/// error mesage.
pub const FAILABLE_CONSTRAINT_NAMES: [&str; 2] = ["inf", "nan"];

/// Utility function to check the "legality" of a constraint name based on the disallowed
/// word beginnings as given in [`FAILABLE_CONSTRAINT_NAMES`]. Simply return a bool to let
/// the caller decide on how to handle this case.
pub fn starts_with_failable(s: &str) -> bool {
    FAILABLE_CONSTRAINT_NAMES
        .iter()
        .any(|prefix| s.to_lowercase().starts_with(&prefix.to_lowercase()))
}

/// A constraint
#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    /// The LHS expression of the constraint.
    pub lhs: Expression,
    /// The RHS of a constraint which can be an arbitrary `Bias` value.
    pub rhs: Bias,
    /// The comparator defines the relationship between the LHS and RHS of the constraint.
    /// See `Comparator` for all available options.
    pub comparator: Comparator,
    /// A Constraint can also be named for easier, more native indexing into a collection of
    /// constraints.
    pub name: String,
}

impl Constraint {
    pub fn deep_clone(&self, env: ArcEnv) -> Self {
        _ = env;
        unimplemented!()
    }
}

use std::{fmt::Display, ops::Sub};

use global_counter::primitive::exact::CounterU64;
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Comparator};

use crate::{ArcEnv, expression::Expression, prelude::VarRef, traits::ContentEquality};

pub static CONSTRAINT_COUNTER: CounterU64 = CounterU64::new(0);

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
    pub fn new(
        lhs: Expression,
        rhs: Bias,
        comparator: Comparator,
        name: Option<String>,
    ) -> LunaModelResult<Self> {
        validate_name(&name)?;
        let lhs_constant = lhs.offset;
        let rhs = rhs - lhs_constant;
        let lhs = lhs.sub(lhs_constant)?;
        Ok(Self {
            lhs,
            rhs,
            comparator,
            name: name.unwrap_or_else(|| format!("c{}", CONSTRAINT_COUNTER.inc())),
        })
    }

    pub fn deep_clone(&self, env: ArcEnv) -> Self {
        _ = env;
        unimplemented!()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// A [VarRef] in our LHS [Expression] can be replaced by a new [Expression] using this function.
    /// It's a convenience function to enable the [substitution](Expression::substitute) operation on
    /// the LHS [Expression] of a constraint. All substitution logic and operations are defined on
    /// the [Expression] [here](Expression::substitute).
    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> LunaModelResult<()> {
        self.lhs = (&self.lhs).substitute(target, replacement)?;
        Ok(())
    }
}

/// Utility function to validate the correctness/legality of a [Constraint] name.
/// A [Constraint] name is considered legal if it is:
/// - not an empty string
/// - the first char is alphabetical
/// - starts with any of the failable word beginnings defined in
/// (FAILABLE_CONSTRAINT_NAMES)[crate::core::constraints::constraint::FAILABLE_CONSTRAINT_NAMES].
/// checked in [starts_with_failable].
/// This function returns an error when an illegal [Constraint] name is given as an argument.
fn validate_name(name: &Option<String>) -> LunaModelResult<()> {
    if let Some(name) = &name {
        if name.is_empty() {
            return Err(LunaModelError::ConstraintNameInvalid(
                "constraint names cannot be empty strings".into(),
            ));
        }
        let first_char = name.chars().next().unwrap();
        let first_char_alpha = first_char.is_alphabetic();

        if !first_char_alpha {
            return Err(LunaModelError::ConstraintNameInvalid(
                format!(
                    "constraint names must start with an alphabetical character, is {}",
                    first_char
                )
                .into(),
            ));
        }

        if starts_with_failable(name) {
            return Err(LunaModelError::ConstraintNameInvalid(
                format!(
                    "constraint names cannot start with one of '{}', is {}",
                    FAILABLE_CONSTRAINT_NAMES.join(", "),
                    name
                )
                .into(),
            ));
        }
    }
    Ok(())
}

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        _ = f;
        unimplemented!()
    }
}

impl ContentEquality for Constraint {
    fn equal_contents(&self, other: &Self) -> bool {
        self.lhs.equal_contents(&other.lhs)
            && self.rhs == other.rhs
            && self.comparator == other.comparator
            && self.name == other.name
    }
}

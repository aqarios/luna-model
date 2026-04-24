use std::ops::Sub;

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

/// A single algebraic constraint.
///
/// Constraints are stored in normalized form: any constant offset already
/// present on the left-hand side expression is moved onto the right-hand side
/// during construction. That keeps later evaluation and translation logic from
/// having to repeatedly special-case a left-hand side constant term.
#[derive(Debug, Clone)]
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
    name: String,
    /// If the constraint name was set automatically, (implicit determined).
    pub auto_name: bool,
}

impl Constraint {
    /// Creates a new constraint and normalizes its left-hand side.
    ///
    /// If `lhs` contains a constant offset, that constant is subtracted from the
    /// expression and folded into `rhs`. The resulting constraint is therefore
    /// easier to compare, serialize, and translate consistently.
    pub fn new(
        lhs: Expression,
        rhs: Bias,
        comparator: Comparator,
        name: Option<String>,
    ) -> LunaModelResult<Self> {
        validate_name(name.as_ref())?;
        let lhs_constant = lhs.offset;
        let rhs = rhs - lhs_constant;
        let lhs = lhs.sub(lhs_constant)?;
        Ok(Self {
            lhs,
            rhs,
            comparator,
            auto_name: name.is_none(),
            name: name.unwrap_or_else(|| format!("c{}", CONSTRAINT_COUNTER.inc())),
        })
    }

    /// Deep-clones the constraint into a different shared environment.
    ///
    /// This keeps the algebraic structure intact while re-rooting the contained
    /// expression so it refers to variables from `env`.
    pub fn deep_clone(&self, env: ArcEnv) -> Self {
        Self {
            lhs: self.lhs.deep_clone(env),
            rhs: self.rhs,
            comparator: self.comparator,
            name: self.name.clone(),
            auto_name: self.auto_name,
        }
    }

    /// Returns the current constraint name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Renames the constraint after validating the new name.
    ///
    /// Calling this marks the constraint as explicitly named, which matters when
    /// collections later decide whether they may auto-generate fallback names.
    pub fn set_name(&mut self, name: String) -> LunaModelResult<()> {
        validate_name(Some(&name))?;
        self.name = name;
        self.auto_name = false;
        Ok(())
    }

    /// A [VarRef] in our LHS [Expression] can be replaced by a new [Expression] using this function.
    /// It's a convenience function to enable the [substitution](Expression::substitute) operation on
    /// the LHS [Expression] of a constraint. All substitution logic and operations are defined on
    /// the [Expression] [here](Expression::substitute).
    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> LunaModelResult<()> {
        self.lhs = self.lhs.substitute(target, replacement)?;
        Ok(())
    }
}

/// Utility function to validate the correctness/legality of a [Constraint] name.
/// A [Constraint] name is considered legal if it is:
/// - not an empty string
/// - the first char is alphabetical
/// - does not start with any of the prefixes listed in [`FAILABLE_CONSTRAINT_NAMES`]
///
/// The last rule exists because some downstream LP/MPS readers misclassify names
/// that begin with special floating-point spellings such as `inf` or `nan`.
/// LunaModel rejects such names early so translation failures become explicit and
/// easier to diagnose.
fn validate_name(name: Option<&String>) -> LunaModelResult<()> {
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

impl ContentEquality for Constraint {
    /// Compares the semantic contents of two constraints while ignoring their names.
    fn equal_contents(&self, other: &Self) -> bool {
        self.lhs.equal_contents(&other.lhs)
            && self.rhs == other.rhs
            && self.comparator == other.comparator
    }
}

impl PartialEq for Constraint {
    /// Compares two constraints including their explicit names.
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs
            && self.rhs == other.rhs
            && self.comparator == other.comparator
            && self.name == other.name
    }
}

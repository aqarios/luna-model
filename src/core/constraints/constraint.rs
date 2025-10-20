use crate::core::expression::{Expression, ExpressionEvaluation};
use crate::core::operations::SubToExpression;
use crate::core::traits::ContentEquality;
use crate::core::writer::ModelWriter;
use crate::core::{ExpressionBase, SharedEnvironment, Substitution, ValueByIndex, VarRef};
use crate::errors::{DifferentEnvsErr, IllegalConstraintNameErr};
use crate::types::{Bias, VarIndex};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Mul;
use std::string::ToString;
use strum_macros::Display;

#[cfg(feature = "py")]
use pyo3::prelude::*;

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

#[cfg_attr(
    all(feature = "py", not(feature = "lq")),
    pyclass(eq, eq_int, name = "Comparator", module = "aqmodels._core")
)]
#[cfg_attr(
    all(feature = "py", feature = "lq"),
    pyclass(eq, eq_int, name = "Comparator", module = "luna_quantum._core")
)]
/// Comparison operators used to define constraints.
///
/// This enum represents the logical relation between the left-hand side (LHS)
/// and the right-hand side (RHS) of a [Constraint].
///
/// Attributes
/// ----------
/// Eq : Comparator
///     Equality constraint (==).
/// Le : Comparator
///     Less-than-or-equal constraint (<=).
/// Ge : Comparator
///     Greater-than-or-equal constraint (>=).
///
/// Examples
/// --------
/// >>> from luna_quantum import Comparator
/// >>> str(Comparator.Eq)
/// '=='
// we require the python config here, since wrapping an enum in the py_bindings is a tedious task.
#[derive(Debug, Copy, Clone, PartialEq, Display, Eq, Hash)]
pub enum Comparator {
    /// The Equality comparison (==) for a constraint where LHS == RHS.
    #[strum(to_string = "==")]
    Eq,
    /// The Less-than or equal comparison (<=) for a constraint where LHS <= RHS.
    #[strum(to_string = "<=")]
    Le,
    /// The Greater-than or equal comparison (>=) for a constraint where LHS >= RHS.
    #[strum(to_string = ">=")]
    Ge,
}

impl Comparator {
    /// Utility function used to determine if the [Constraint] is met given concrete LHS
    /// and RHS values. This function is used as part of the [Model](crate::core::Model) evaluation of samples
    /// or a solution.
    pub fn evaluate(&self, lhs: Bias, rhs: Bias) -> bool {
        match self {
            Self::Eq => lhs == rhs,
            Self::Le => lhs <= rhs,
            Self::Ge => lhs >= rhs,
        }
    }
}

/// A constraint is a collection of
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
    pub name: String,
}

impl Constraint {
    /// A constraint contains an [Expression] for the LHS which in turn has a shared reference
    /// to an [Environment](crate::core::Environment) (specifically a [SharedEnvironment]). Simply cloning a [Constraint]
    /// results in a copy/clone of most of the data with a reference to the same [SharedEnvironment]
    /// as the cloned constraint
    /// To get a free-standing copy/clone independent of the original [SharedEnvironment], this
    /// function can be used with a new [SharedEnvironment] the [Expression]s should reference to.
    pub fn deep_clone(&self, env: SharedEnvironment) -> Self {
        Self {
            lhs: self.lhs.deep_clone(env),
            rhs: self.rhs,
            comparator: self.comparator,
            name: self.name.clone(),
        }
    }

    #[inline]
    pub fn has_placeholder_name(&self) -> bool {
        self.name == DEFAULT_CONSTRAINT_NAME
    }

    #[inline]
    pub fn set_name_for_idx(&mut self, idx: usize) {
        self.name = format!("c{}", idx);
    }
}

impl Constraint {
    /// Create a new constraint.
    ///
    /// This function will change from accepting an `Option<String>` to a `String` for the name
    /// with <https://github.com/aqarios/aq-models-rs/400>.
    pub fn new(
        lhs: Expression,
        rhs: Bias,
        comparator: Comparator,
        name: Option<String>,
    ) -> Result<Self, IllegalConstraintNameErr> {
        Self::validate_name(&name)?;
        let lhs_constant = lhs.offset();
        let actual_rhs = rhs - lhs_constant;
        let lhs = lhs.sub(lhs_constant);
        Ok(Self {
            lhs,
            rhs: actual_rhs,
            comparator,
            name: name.unwrap_or_else(|| DEFAULT_CONSTRAINT_NAME.to_string()),
        })
    }

    /// Utility function to validate the correctness/legality of a [Constraint] name.
    /// A [Constraint] name is considered legal if it is:
    /// - not an empty string
    /// - the first char is alphabetical
    /// - starts with any of the failable word beginnings defined in
    /// (FAILABLE_CONSTRAINT_NAMES)[crate::core::constraints::constraint::FAILABLE_CONSTRAINT_NAMES].
    /// checked in [starts_with_failable].
    /// This function returns an error when an illegal [Constraint] name is given as an argument.
    pub fn validate_name(name: &Option<String>) -> Result<(), IllegalConstraintNameErr> {
        if let Some(name) = &name {
            if name.is_empty() {
                return Err(IllegalConstraintNameErr(
                    "constraint names cannot be empty strings".to_string(),
                ));
            }
            let first_char = name.chars().next().unwrap();
            let first_char_alpha = first_char.is_alphabetic();

            if !first_char_alpha {
                return Err(IllegalConstraintNameErr(format!(
                    "constraint names must start with an alphabetical character, is {}",
                    first_char
                )));
            }

            if starts_with_failable(name) {
                return Err(IllegalConstraintNameErr(format!(
                    "constraint names cannot start with one of '{}', is {}",
                    FAILABLE_CONSTRAINT_NAMES.join(", "),
                    name
                )));
            }
        }
        Ok(())
    }

    /// Set the name of the [Constraint]. If the [Constraint] has a name set already. The old name
    /// is overwritten. In case the new name is illegal an error ([IllegalConstraintNameErr]) is returned.
    pub fn set_name(&mut self, name: String) -> Result<(), IllegalConstraintNameErr> {
        Self::validate_name(&Some(name.clone()))?;
        self.name = name;
        Ok(())
    }

    /// To check if a [Constraint] (`self`) is satisfied given a concrete Sample.
    /// Since we cannot ensure the data contained in the Sample is aligned with the variables
    /// of the [Constraint]'s expression, this function also requires an index map for computing
    /// the actual value of an [Expression] for the given Sample. This `index_map` maps the
    /// [VarRef]'s index to the position in the Sample.
    pub fn evaluate_sample<'a, Elem: 'a, Sample: ValueByIndex<VarIndex, Output = Elem>, F>(
        &self,
        sample: &'a Sample,
        index_map: F,
    ) -> bool
    where
        Elem: Mul<Bias, Output = Bias>,
        F: Fn(VarIndex) -> VarIndex,
    {
        let val = self.lhs.evaluate_sample(sample, &index_map);
        self.comparator.evaluate(val, self.rhs)
    }

    /// A [VarRef] in our LHS [Expression] can be replaced by a new [Expression] using this function.
    /// It's a convenience function to enable the [substitution](Expression::substitute) operation on
    /// the LHS [Expression] of a constraint. All substitution logic and operations are defined on
    /// the [Expression] [here](Expression::substitute).
    pub fn substitute(
        &mut self,
        target: &VarRef,
        replacement: &Expression,
    ) -> Result<(), DifferentEnvsErr> {
        self.lhs = (&self.lhs).substitute(target, replacement)?;
        Ok(())
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = ModelWriter::new().write_constraint(&self).to_string();
        f.write_str(&s)
    }
}

impl ContentEquality for Constraint {
    /// Since the [Constraint] (indirectly) has a reference to a [SharedEnvironment] we cannot simply
    /// check by equality using the builtin (derived) equality primitives. Since two [Constraint]s
    /// might be equal albeit being defined in different [SharedEnvironment]s. We still want to maintain
    /// the option to check if two [Constraint]s are identical (including the [SharedEnvironment])
    /// so we define this function to keep the `==` operator for identity checks.
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.lhs.is_equal_contents(&other.lhs)
            && self.rhs == other.rhs
            && self.comparator == other.comparator
            && self.name == other.name
    }
}

impl PartialEq for Constraint {
    /// Check if two [Constraint]s are identical using `==` operation.
    fn eq(&self, other: &Self) -> bool {
        self.comparator == other.comparator && self.rhs == other.rhs && self.lhs == other.lhs
    }
}

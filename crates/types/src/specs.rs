use std::fmt::Display;

use enumset::{EnumSet, EnumSetType};
use strum_macros::{Display, EnumString};

use crate::{Sense, Vtype, utils::EnumSetFromVec};

/// Constraint families used when describing model capabilities.
#[derive(EnumSetType, Display, Debug, Hash, EnumString)]
pub enum Ctype {
    /// Only equality constraints allowed.
    #[strum(to_string = "Unconstrained")]
    Unconstrained,
    /// Only equality constraints allowed.
    #[strum(to_string = "Equality")]
    Equality,
    /// Only inequality constraints allowed.
    #[strum(to_string = "Inequality")]
    Inequality,
    /// Only ge-inequality constraints allowed.
    #[strum(to_string = "LessEqual")]
    LessEqual,
    /// Only le-inequality constraints allowed.
    #[strum(to_string = "GreaterEqual")]
    GreaterEqual,
}

impl EnumSetFromVec<Ctype> for Vec<Ctype> {
    /// Converts a list of constraint categories into a hierarchical [`EnumSet`].
    ///
    /// `Inequality` implies both directional inequality variants, so the output
    /// set expands that relationship explicitly.
    fn to_enumset(&self) -> EnumSet<Ctype> {
        use Ctype::*;
        let mut es = EnumSet::default();
        for entry in self.iter() {
            match entry {
                Inequality => es.insert_all(LessEqual | GreaterEqual | Inequality),
                _ => _ = es.insert(*entry),
            }
        }
        es
    }
}

/// Compact summary of what kinds of models a workflow can accept or produce.
///
/// The fields are optional because many analyses and passes only care about a
/// subset of the available properties.
#[derive(Debug, Clone)]
pub struct Specs {
    /// Required or known optimization sense.
    pub sense: Option<Sense>,
    /// Allowed or observed variable types.
    pub vtypes: Option<EnumSet<Vtype>>,
    /// Allowed or observed constraint categories.
    pub constraints: Option<EnumSet<Ctype>>,
    /// Upper bound on objective degree.
    pub max_degree: Option<usize>,
    /// Upper bound on constraint degree.
    pub max_constraint_degree: Option<usize>,
    /// Upper bound on number of variables.
    pub max_num_variables: Option<usize>,
}

impl Specs {
    /// Creates a fully specified capability summary.
    pub fn new(
        sense: Sense,
        vtypes: EnumSet<Vtype>,
        constraints: EnumSet<Ctype>,
        max_degree: usize,
        max_constraint_degree: usize,
        max_num_variables: usize,
    ) -> Self {
        Self {
            sense: Some(sense),
            vtypes: Some(vtypes),
            constraints: Some(constraints),
            max_degree: Some(max_degree),
            max_constraint_degree: Some(max_constraint_degree),
            max_num_variables: Some(max_num_variables),
        }
    }

    /// Returns whether `self` satisfies all requirements expressed by `other`.
    ///
    /// `None` in `other` means "do not care". `None` in `self` means the
    /// property is unknown or unspecified, which is only acceptable if `other`
    /// also does not require that property.
    pub fn satisfies(&self, other: &Self) -> bool {
        if !check_spec_eq(self.sense, other.sense) {
            return false;
        }
        if !check_spec_enumset(self.vtypes, other.vtypes) {
            return false;
        }
        if !check_spec_enumset(self.constraints, other.constraints) {
            return false;
        }
        if !check_spec_le(self.max_degree, other.max_degree) {
            return false;
        }
        if !check_spec_le(self.max_constraint_degree, other.max_constraint_degree) {
            return false;
        }
        if !check_spec_le(self.max_num_variables, other.max_num_variables) {
            return false;
        }
        true
    }
}

/// Checks equality-style requirements on optional fields.
fn check_spec_eq<T: PartialEq>(a: Option<T>, b: Option<T>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a == b,
        (None, Some(_)) => false, // the given spec `other` cares, but self doesn't => not ok.
        (Some(_), None) => true,  // the given spec `other` doesn't care.
        (None, None) => true,     // both don't specify the spec.
    }
}

/// Checks upper-bound-style requirements on optional fields.
fn check_spec_le<T: PartialOrd>(a: Option<T>, b: Option<T>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a <= b,
        (None, Some(_)) => false, // the given spec `other` cares, but self doesn't => not ok.
        (Some(_), None) => true,  // the given spec `other` doesn't care.
        (None, None) => true,     // both don't specify the spec.
    }
}

/// Checks set containment requirements on optional [`EnumSet`] fields.
fn check_spec_enumset<T: EnumSetType>(a: Option<EnumSet<T>>, b: Option<EnumSet<T>>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => (a | b) == b,
        (None, Some(_)) => false, // the given spec `other` cares, but self doesn't => not ok.
        (Some(_), None) => true,  // the given spec `other` doesn't care.
        (None, None) => true,     // both don't specify the spec.
    }
}

impl Display for Specs {
    /// Formats the capability summary in a developer-oriented one-line form.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ModelSpecs(sense={}, vtype={}, constraints={}, max_degree={}, max_constraint_degree={}, max_num_variables={})",
            self.sense.map_or_else(|| String::from("None"), |v| v.to_string()),
            self.vtypes.map_or_else(|| String::from("None"), |v| v.to_string()),
            self.constraints.map_or_else(|| String::from("None"), |v| v.to_string()),
            self.max_degree.map_or_else(|| String::from("None"), |v| v.to_string()),
            self.max_constraint_degree.map_or_else(|| String::from("None"), |v| v.to_string()),
            self.max_num_variables.map_or_else(|| String::from("None"), |v| v.to_string()),
        ))
    }
}

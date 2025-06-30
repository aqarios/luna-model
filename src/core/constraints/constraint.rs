use crate::core::expression::{Expression, ExpressionEvaluation};
use crate::core::operations::SubToExpression;
use crate::core::traits::ContentEquality;
use crate::core::writer::ModelWriter;
use crate::core::{ExpressionBase, SharedEnvironment, Substitution, ValueByIndex, VarRef};
use crate::errors::{
    DifferentEnvsErr, DuplicateConstraintNameErr, IllegalConstraintNameErr, IndexOutOfBoundsErr,
};
use crate::types::{Bias, VarIndex};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul};
use std::slice::Iter;
use std::string::ToString;
use strum_macros::Display;

#[cfg(feature = "py")]
use pyo3::prelude::*;

const FAILABLE_CONSTRAINT_NAMES: [&str; 2] = ["inf", "nan"];

fn starts_with_failable(s: &str) -> bool {
    FAILABLE_CONSTRAINT_NAMES
        .iter()
        .any(|prefix| s.to_lowercase().starts_with(&prefix.to_lowercase()))
}

/// Comparison operators used to define constraints.
///
/// This enum represents the logical relation between the left-hand side (LHS)
/// and the right-hand side (RHS) of a constraint.
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
#[cfg_attr(
    all(feature = "py", not(feature = "lq")),
    pyclass(eq, eq_int, name = "Comparator", module = "aqmodels")
)] 
#[cfg_attr(
    all(feature = "py", feature = "lq"),
    pyclass(eq, eq_int, name = "Comparator", module = "luna_quantum")
)] 
#[derive(Debug, Copy, Clone, PartialEq, Display)]
pub enum Comparator {
    /// Equality (==)
    #[strum(to_string = "==")]
    Eq,
    /// Less-than or equal (<=)
    #[strum(to_string = "<=")]
    Le,
    /// Greater-than or equal (>=)
    #[strum(to_string = ">=")]
    Ge,
}

impl Comparator {
    pub fn evaluate(&self, lhs: Bias, rhs: Bias) -> bool {
        match self {
            Self::Eq => lhs == rhs,
            Self::Le => lhs <= rhs,
            Self::Ge => lhs >= rhs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Constraint {
    // todo, expression in constraint should be immutable...
    pub lhs: Expression,
    pub rhs: Bias,
    pub comparator: Comparator,
    pub name: Option<String>,
}

impl Constraint {
    /// Deep clone a constraint for the new environment.
    pub fn deep_clone(&self, env: SharedEnvironment) -> Self {
        Self {
            lhs: self.lhs.deep_clone(env),
            rhs: self.rhs,
            comparator: self.comparator,
            name: self.name.clone(),
        }
    }
}

impl Constraint {
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
            name,
        })
    }

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

    pub fn set_name(&mut self, name: Option<String>) -> Result<(), IllegalConstraintNameErr> {
        Self::validate_name(&name)?;
        self.name = name;
        Ok(())
    }

    pub fn evaluate_sample<'a, Elem: 'a, Sample: ValueByIndex<VarIndex, Output = Elem>>(
        &self,
        sample: &'a Sample,
    ) -> bool
    where
        Elem: Mul<Bias, Output = Bias>,
    {
        let val = self.lhs.evaluate_sample(sample);
        self.comparator.evaluate(val, self.rhs)
    }

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
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.lhs.is_equal_contents(&other.lhs)
            && self.rhs == other.rhs
            && self.comparator == other.comparator
            && self.name == other.name
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constraints {
    pub used_names: Vec<String>,
    pub constraints: Vec<Constraint>,
}

impl Constraints {
    /// Deep clone the constraints for the new environment.
    pub fn deep_clone(&self, env: SharedEnvironment) -> Self {
        Self {
            used_names: self.used_names.clone(),
            constraints: self
                .constraints
                .iter()
                .map(|c| c.deep_clone(env.clone()))
                .collect(),
        }
    }
}

impl Constraints {
    pub fn default() -> Self {
        Self {
            used_names: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn new_from(other: &Self) -> Self {
        Self {
            used_names: other.used_names.clone(),
            constraints: other.constraints.clone(),
        }
    }

    pub fn new_from_vec(constraints: Vec<Constraint>) -> Self {
        Self {
            used_names: constraints
                .iter()
                .filter(|c| c.name.is_some())
                .map(|c| c.name.clone().unwrap())
                .collect(),
            constraints,
        }
    }

    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    pub fn iter(&self) -> Iter<'_, Constraint> {
        self.constraints.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_constraint(&self, index: usize) -> Result<&Constraint, IndexOutOfBoundsErr> {
        if index >= self.len() {
            return Err(IndexOutOfBoundsErr::new(index, self.len()));
        }
        Ok(&self.constraints[index])
    }

    pub fn substitute(
        &mut self,
        target: &VarRef,
        replacement: &Expression,
    ) -> Result<(), DifferentEnvsErr> {
        for constr in self.constraints.iter_mut() {
            constr.substitute(target, replacement)?;
        }
        Ok(())
    }
}

impl Add<Constraint> for Constraints {
    type Output = Result<Constraints, DuplicateConstraintNameErr>;

    fn add(self, rhs: Constraint) -> Self::Output {
        let mut out = Constraints::new_from(&self);
        out.add_assign(&rhs)?;
        Ok(out)
    }
}

impl Constraints {
    pub fn add_assign(&mut self, rhs: &Constraint) -> Result<(), DuplicateConstraintNameErr> {
        if let Some(name) = &rhs.name {
            if self.used_names.contains(&name) {
                return Err(DuplicateConstraintNameErr(name.to_string()));
            } else {
                self.used_names.push(name.to_string())
            }
        }
        Ok(self.constraints.push(rhs.clone()))
    }
}

impl Add<&Constraint> for &Constraints {
    type Output = Result<Constraints, DuplicateConstraintNameErr>;

    fn add(self, rhs: &Constraint) -> Self::Output {
        if rhs.name.is_some() && self.used_names.contains(rhs.name.as_ref().unwrap()) {
            return Err(DuplicateConstraintNameErr(
                rhs.name.as_ref().unwrap().to_string(),
            ));
        }
        let mut out = Constraints::new_from(&self);
        out.add_assign(rhs)?;
        Ok(out)
    }
}

impl PartialEq for Constraint {
    fn eq(&self, other: &Self) -> bool {
        self.comparator == other.comparator && self.rhs == other.rhs && self.lhs == other.lhs
    }
}

impl Display for Constraints {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = ModelWriter::new().write_constraints(&self).to_string();
        f.write_str(&s)
    }
}

impl ContentEquality for Constraints {
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.used_names == other.used_names
            && self.constraints.is_equal_contents(&other.constraints)
    }
}

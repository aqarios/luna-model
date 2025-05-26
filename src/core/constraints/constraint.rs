use crate::core::expression::{BiasConstraints, ExpressionEvaluation, IndexConstraints};
use crate::core::operations::SubAssignToExpression;
use crate::core::writer::ModelWriter;
use crate::core::{ExpressionBase, MutRcExpression, ValueByIndex};
use crate::errors::{IllegalConstraintNameErr, IndexOutOfBoundsErr};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul};
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
#[cfg_attr(
    feature = "py",
    pyclass(eq, eq_int, name = "Comparator", module = "aqmodels")
)] // we require the python config here, since wrapping an enum in the py_bindings is a tedious task.
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
    pub fn evaluate<Bias: BiasConstraints>(&self, lhs: Bias, rhs: Bias) -> bool {
        match self {
            Self::Eq => lhs == rhs,
            Self::Le => lhs <= rhs,
            Self::Ge => lhs >= rhs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Constraint<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    // todo, expression in constraint should be immutable...
    pub lhs: MutRcExpression<Index, Bias>,
    pub rhs: Bias,
    pub comparator: Comparator,
    pub name: Option<String>,
}

impl<Index, Bias> Constraint<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub fn new(
        lhs: MutRcExpression<Index, Bias>,
        rhs: Bias,
        comparator: Comparator,
        name: Option<String>,
    ) -> Result<Self, IllegalConstraintNameErr> {
        Self::validate_name(&name)?;
        let lhs_constant = lhs.borrow().offset();
        let actual_rhs = rhs - lhs_constant;
        lhs.borrow_mut().sub_assign(lhs_constant);
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

    pub fn evaluate_sample<'a, Elem: 'a, Sample: ValueByIndex<Index, Output = Elem>>(
        &self,
        sample: &'a Sample,
    ) -> bool
    where
        Elem: Mul<Bias, Output = Bias>,
    {
        let val = self.lhs.borrow().evaluate_sample(sample);
        self.comparator.evaluate(val, self.rhs)
    }
}

impl<Index, Bias> Display for Constraint<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = ModelWriter::new().write_constraint(&self).to_string();
        f.write_str(&s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub constraints: Vec<Constraint<Index, Bias>>,
}

impl<Index, Bias> Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub fn default() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    pub fn new_from(other: &Self) -> Self {
        Self {
            constraints: other.constraints.clone(),
        }
    }

    pub fn new_from_vec(constraints: Vec<Constraint<Index, Bias>>) -> Self {
        Self { constraints }
    }

    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    pub fn iter(&self) -> Iter<'_, Constraint<Index, Bias>> {
        self.constraints.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_constraint(
        &self,
        index: usize,
    ) -> Result<&Constraint<Index, Bias>, IndexOutOfBoundsErr> {
        if index >= self.len() {
            return Err(IndexOutOfBoundsErr::new(index, self.len()));
        }
        Ok(&self.constraints[index])
    }
}

impl<Index, Bias> AddAssign<Constraint<Index, Bias>> for Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn add_assign(&mut self, rhs: Constraint<Index, Bias>) {
        self.constraints.push(rhs)
    }
}

impl<Index, Bias> Add<Constraint<Index, Bias>> for Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Constraints<Index, Bias>;

    fn add(self, rhs: Constraint<Index, Bias>) -> Self::Output {
        let mut out = Constraints::new_from(&self);
        out += rhs;
        out
    }
}

impl<Index, Bias> AddAssign<&Constraint<Index, Bias>> for Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn add_assign(&mut self, rhs: &Constraint<Index, Bias>) {
        self.constraints.push(rhs.clone());
    }
}

impl<Index, Bias> Add<&Constraint<Index, Bias>> for &Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Constraints<Index, Bias>;

    fn add(self, rhs: &Constraint<Index, Bias>) -> Self::Output {
        let mut out = Constraints::new_from(&self);
        out += rhs;
        out
    }
}

impl<Index, Bias> PartialEq for Constraint<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn eq(&self, other: &Self) -> bool {
        self.comparator == other.comparator && self.rhs == other.rhs && self.lhs == other.lhs
    }
}

impl<Index, Bias> Display for Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = ModelWriter::new().write_constraints(&self).to_string();
        f.write_str(&s)
    }
}

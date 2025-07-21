use crate::core::expression::{Expression, ExpressionEvaluation};
use crate::core::operations::SubToExpression;
use crate::core::traits::ContentEquality;
use crate::core::writer::ModelWriter;
use crate::core::{ExpressionBase, SharedEnvironment, Substitution, ValueByIndex, VarRef};
use crate::errors::{
    DifferentEnvsErr, DuplicateConstraintNameErr, GetConstraintErr, IllegalConstraintNameErr,
    IndexOutOfBoundsErr,
};
use crate::types::{Bias, VarIndex};
use indexmap::IndexMap;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul};
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
    pyclass(eq, eq_int, name = "Comparator", module = "aqmodels._core")
)]
#[cfg_attr(
    all(feature = "py", feature = "lq"),
    pyclass(eq, eq_int, name = "Comparator", module = "luna_quantum._core")
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
    pub index_map: IndexMap<String, usize>,
    pub constraints: Vec<Constraint>,
}

#[cfg_attr(feature = "py", derive(FromPyObject))]
pub enum ConstraintKey {
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

impl Constraints {
    /// Deep clone the constraints for the new environment.
    pub fn deep_clone(&self, env: SharedEnvironment) -> Self {
        Self {
            index_map: self.index_map.clone(),
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
            index_map: IndexMap::new(),
            constraints: Vec::new(),
        }
    }

    pub fn new_from(other: &Self) -> Self {
        Self {
            index_map: other.index_map.clone(),
            constraints: other.constraints.clone(),
        }
    }

    pub fn new_from_vec(constraints: Vec<Constraint>) -> Self {
        let mut slf = Self::default();
        constraints.into_iter().enumerate().for_each(|(idx, c)| {
            let name = (&c).name.clone().unwrap_or(format!("c{idx}").to_string());
            slf.index_map.insert(name.clone(), idx);
            slf.constraints.push(c);
        });
        slf
    }

    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    pub fn iter(&self) -> ConstraintsIterator {
        ConstraintsIterator::new(&self)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_constraint(&self, key: ConstraintKey) -> Result<&Constraint, GetConstraintErr> {
        let index = match &key {
            ConstraintKey::Int(idx) => Some(idx),
            ConstraintKey::Str(name) => self.index_map.get(name),
        };
        match index {
            Some(idx) => {
                let constr = self.constraints.get(*idx);
                match constr {
                    Some(constr) => Ok(constr),
                    None => Err(GetConstraintErr::IndexOutOfBoundsErr(
                        IndexOutOfBoundsErr::new(*idx, self.len()),
                    )),
                }
            }
            None => Err(GetConstraintErr::NoConstraintForKeyErr(key.to_string())),
        }
    }

    pub fn remove_constraint(&mut self, key: ConstraintKey) {
        let (idx, name) = match &key {
            ConstraintKey::Int(idx) => {
                let v = self.index_map.iter().find(|(_, b)| **b == *idx);
                if let Some((name, _)) = v {
                    (*idx, name.clone())
                } else {
                    return ();
                }
            }
            ConstraintKey::Str(name) => {
                if let Some(idx) = self.index_map.get(name) {
                    (*idx, name.clone())
                } else {
                    return ();
                }
            }
        };
        self.constraints.remove(idx);
        self.index_map.shift_remove(&name);
        self.index_map
            .iter_mut()
            .filter(|(_, index)| **index > idx)
            .for_each(|(_, index)| *index -= 1);
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
            if self.index_map.contains_key(name) {
                return Err(DuplicateConstraintNameErr(name.to_string()));
            } else {
                self.index_map
                    .insert(name.to_string(), self.constraints.len());
            }
        } else {
            let idx = self.constraints.len();
            self.index_map.insert(format!("c{}", idx), idx);
        }
        Ok(self.constraints.push(rhs.clone()))
    }
}

impl Add<&Constraint> for &Constraints {
    type Output = Result<Constraints, DuplicateConstraintNameErr>;

    fn add(self, rhs: &Constraint) -> Self::Output {
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
        self.index_map == other.index_map && self.constraints.is_equal_contents(&other.constraints)
    }
}

pub struct ConstraintsIterator<'a> {
    collection: &'a Constraints,
    names: Vec<&'a String>,
    indices: Vec<usize>,
    current: usize,
}

impl<'a> ConstraintsIterator<'a> {
    fn new(collection: &'a Constraints) -> Self {
        let mut names = Vec::new();
        let mut indices = Vec::new();
        collection.index_map.iter().for_each(|(key, idx)| {
            names.push(key);
            indices.push(*idx);
        });
        Self {
            collection,
            current: 0,
            indices,
            names,
        }
    }
}

impl<'a> Iterator for ConstraintsIterator<'a> {
    type Item = (&'a String, &'a Constraint);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.names.len() {
            return None;
        }
        let name = self.names[self.current];
        let index = self.indices[self.current];
        let constr = &self.collection.constraints[index];
        self.current += 1;
        Some((name, constr))
    }
}

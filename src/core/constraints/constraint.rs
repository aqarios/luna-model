use crate::core::utils::ModelWriter;
use crate::core::{
    expression::{BiasConstraints, IndexConstraints},
    Expression,
};
use std::fmt::{Debug, Display, Formatter};
use std::slice::Iter;
use std::string::ToString;
use std::{
    cell::{Ref, RefCell},
    ops::{Add, AddAssign},
    rc::Rc,
};
use strum_macros::Display;

#[derive(Debug, Copy, Clone, PartialEq, Display)]
pub enum Comparator {
    #[strum(to_string = "==")]
    Eq,
    #[strum(to_string = "<=")]
    Leq,
    #[strum(to_string = ">=")]
    Geq,
}

#[derive(Debug, Clone)]
pub struct Constraint<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    // todo, expression in constraint should be immutable...
    pub lhs: Rc<RefCell<Expression<Index, Bias>>>,
    pub rhs: Bias,
    pub comparator: Comparator,
}

impl<Index, Bias> Constraint<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub fn new(
        lhs: Rc<RefCell<Expression<Index, Bias>>>,
        rhs: Bias,
        comparator: Comparator,
    ) -> Self {
        Self {
            lhs,
            rhs,
            comparator,
        }
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

impl<Index, Bias> AddAssign<Ref<'_, Constraint<Index, Bias>>> for Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn add_assign(&mut self, rhs: Ref<'_, Constraint<Index, Bias>>) {
        self.constraints.push(rhs.clone());
    }
}

impl<Index, Bias> Add<Ref<'_, Constraint<Index, Bias>>> for &Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = Constraints<Index, Bias>;

    fn add(self, rhs: Ref<'_, Constraint<Index, Bias>>) -> Self::Output {
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

// impl<Index, Bias> PartialEq for Constraints<Index, Bias>
// where
//     Index: IndexConstraints,
//     Bias: BiasConstraints,
// {
//     fn eq(&self, other: &Self) -> bool {
//         let mut num_matches = 0;
//         for lhs in self.constraints.iter() {
//             for rhs in other.constraints.iter() {
//                 num_matches += (lhs == rhs) as usize;
//             }
//         }
//         num_matches >= self.constraints.len()
//     }
// }

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

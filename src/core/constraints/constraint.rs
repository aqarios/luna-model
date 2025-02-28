use std::{
    cell::{Ref, RefCell},
    ops::{Add, AddAssign},
    rc::Rc,
};

use crate::core::{
    expression::{BiasConstraints, IndexConstraints},
    Expression,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Comparator {
    Eq,
    Leq,
    Geq,
}

#[derive(Debug, Clone)]
pub struct Constraint<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    // hmm, expression in constraint should be immutable...
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

#[derive(Debug, Clone)]
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

impl<Index, Bias> PartialEq for Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn eq(&self, other: &Self) -> bool {
        let mut num_matches = 0;
        for lhs in self.constraints.iter() {
            for rhs in other.constraints.iter() {
                num_matches += (lhs == rhs) as usize;
            }
        }
        num_matches >= self.constraints.len()
    }
}

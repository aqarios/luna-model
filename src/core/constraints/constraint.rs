use std::{
    cell::RefCell,
    ops::{Add, AddAssign},
    rc::Rc,
};

use crate::core::{
    expression::{BiasConstraints, IndexConstraints},
    Expression,
};

#[derive(Copy, Clone)]
pub enum Comparator {
    Eq,
    Leq,
    Geq,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    constraints: Vec<Constraint<Index, Bias>>,
}

impl<Index, Bias> Constraints<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    // pub fn default() -> Self {
    //     Self {
    //         constraints: Vec::new(),
    //     }
    // }

    pub fn new_from(other: &Self) -> Self {
        Self {
            constraints: other.constraints.clone(),
        }
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

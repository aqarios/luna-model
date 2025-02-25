use std::{
    cmp::max,
    ops::{Index, IndexMut, MulAssign},
    slice::{Iter, IterMut},
};

use crate::core::expression::{BiasConstraints, IndexConstraints};

use super::types::{OneVarTerm, OneVarTermConstruction};

// todo: we need a Linear trait to allow for better interchangeability...
// Currently the expression traits use the structs directly. I don't like this...

#[derive(Clone, Debug)]
pub struct LinearDynamic<Index, Bias> {
    biases: Vec<OneVarTerm<Index, Bias>>,
}

impl<Index, Bias> LinearDynamic<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub fn default() -> Self {
        Self { biases: Vec::new() }
    }

    pub fn with_size(size: usize) -> Self {
        let biases = Vec::with_capacity(size);
        // biases.resize(size, Bias::default());
        Self { biases }
    }

    pub fn new_from_weighted_variable(var: usize, bias: Bias) -> Self {
        let mut out = Self::default();
        out.biases.insert(var, OneVarTerm::new(var.into(), bias));
        out
    }

    pub fn new_from_variables(lhs: usize, rhs: usize, bias: Bias) -> Self {
        let mut out = Self::with_size(max(lhs, rhs) + 1);
        out.biases.insert(lhs, OneVarTerm::new(lhs.into(), bias));
        out.biases.insert(rhs, OneVarTerm::new(rhs.into(), bias));
        out
    }

    pub fn iter(&self) -> Iter<OneVarTerm<Index, Bias>> {
        self.biases.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<OneVarTerm<Index, Bias>> {
        let mutvec: &mut Vec<OneVarTerm<Index, Bias>> = self.biases.as_mut();
        mutvec.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.biases.len()
    }

    pub fn resize(&mut self, new_len: usize) {
        // self.biases.resize(new_len, Bias::default());
        // does nothing here...
    }
}

impl<Index, Bias> MulAssign<Bias> for LinearDynamic<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn mul_assign(&mut self, rhs: Bias) {
        for b in self.biases.iter_mut() {
            b.bias *= rhs;
        }
    }
}

impl<Index, Bias> From<&Vec<Bias>> for LinearDynamic<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn from(value: &Vec<Bias>) -> Self {
        Self {
            biases: value
                .iter()
                .enumerate()
                .map(|(i, b)| OneVarTerm::new(i.into(), *b))
                .collect(),
        }
    }
}

impl<Idx, Bias> Index<usize> for LinearDynamic<Idx, Bias>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
{
    type Output = OneVarTerm<Idx, Bias>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.biases[index]
    }
}

impl<Idx, Bias> IndexMut<usize> for LinearDynamic<Idx, Bias>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.biases[index]
    }
}

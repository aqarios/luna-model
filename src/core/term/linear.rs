use crate::core::expression::{BiasConstraints, IndexConstraints};
use crate::core::Environment;
use std::cell::Ref;
use std::{
    cmp::max,
    iter::Enumerate,
    ops::{Index, IndexMut, MulAssign},
    slice::{Iter, IterMut},
};
// todo: we need a Linear trait to allow for better interchangeability...
// Currently the expression traits use the structs directly. I don't like this...

#[derive(Clone, Debug)]
pub struct Linear<Bias> {
    biases: Vec<Bias>,
}

impl<Bias> Linear<Bias>
where
    Bias: BiasConstraints,
{
    pub fn default() -> Self {
        Self { biases: Vec::new() }
    }

    pub fn with_size(size: usize) -> Self {
        let mut biases = Vec::with_capacity(size);
        biases.resize_with(size, Bias::default);
        Self { biases }
    }

    pub fn new_from_weighted_variable(var: usize, bias: Bias) -> Self {
        let mut out = Self::with_size(var + 1);
        out[var] += bias;
        out
    }

    pub fn new_from_variables(lhs: usize, rhs: usize, bias: Bias) -> Self {
        let mut out = Self::with_size(max(lhs, rhs) + 1);
        out[lhs] += bias;
        out[rhs] += bias;
        out
    }

    pub fn iter(&self) -> Enumerate<Iter<Bias>> {
        self.biases.iter().enumerate()
    }

    pub fn iter_mut(&mut self) -> Enumerate<IterMut<Bias>> {
        let mutvec: &mut Vec<Bias> = self.biases.as_mut();
        mutvec.iter_mut().enumerate()
    }

    pub fn len(&self) -> usize {
        self.biases.len()
    }

    pub fn resize(&mut self, new_len: usize) {
        self.biases.resize_with(new_len, Bias::default);
    }

    pub fn display<Index>(&self, env: Ref<Environment<Index>>) -> String
    where
        Index: IndexConstraints,
    {
        let mut out = String::new();
        for (i, bias) in self.iter() {
            if *bias != Bias::zero() {
                let vname = &env.variables[i].name;
                out += &format!(" {} {vname}", bias.to_bias_string());
            }
        }
        out
    }
}

impl<Bias> MulAssign<Bias> for Linear<Bias>
where
    Bias: BiasConstraints,
{
    fn mul_assign(&mut self, rhs: Bias) {
        for b in self.biases.iter_mut() {
            *b *= rhs;
        }
    }
}

impl<Bias> From<&Vec<Bias>> for Linear<Bias>
where
    Bias: Clone,
{
    fn from(value: &Vec<Bias>) -> Self {
        Self {
            biases: value.to_vec(),
        }
    }
}

// todo@benjamin: add the indexing functionality for 'Index' generic.
impl<Bias> Index<usize> for Linear<Bias> {
    type Output = Bias;

    fn index(&self, index: usize) -> &Self::Output {
        &self.biases[index]
    }
}

impl<Bias> IndexMut<usize> for Linear<Bias> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.biases[index]
    }
}

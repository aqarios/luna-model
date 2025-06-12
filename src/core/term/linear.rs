use crate::types::Bias;
use std::{
    cmp::max,
    iter::Enumerate,
    ops::{Index, IndexMut, MulAssign, Neg},
    slice::{Iter, IterMut},
};
// todo: we need a Linear trait to allow for better interchangeability...
// Currently the expression traits use the structs directly. I don't like this...

#[derive(Clone, Debug)]
pub struct Linear {
    biases: Vec<Bias>,
}

impl Linear
{
    pub fn default() -> Self {
        Self { biases: Vec::new() }
    }

    pub fn new(biases: Vec<Bias>) -> Self {
        Self { biases }
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

    pub fn new_from_variables(lhs: (usize, Bias), rhs: (usize, Bias)) -> Self {
        let mut out = Self::with_size(max(lhs.0, rhs.0) + 1);
        out[lhs.0] += lhs.1;
        out[rhs.0] += rhs.1;
        out
    }

    pub fn to_vec(&self) -> &Vec<Bias> {
        &self.biases
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

    pub fn is_zero(&self) -> bool {
        let mut all_zero = true;
        for &b in self.biases.iter() {
            all_zero &= b == Bias::default();
        }
        all_zero
    }
}

impl MulAssign<Bias> for Linear
{
    fn mul_assign(&mut self, rhs: Bias) {
        for b in self.biases.iter_mut() {
            *b *= rhs;
        }
    }
}

impl From<&Vec<Bias>> for Linear
{
    fn from(value: &Vec<Bias>) -> Self {
        Self {
            biases: value.to_vec(),
        }
    }
}

// todo@benjamin: add the indexing functionality for 'Index' generic.
impl Index<usize> for Linear {
    type Output = Bias;

    fn index(&self, index: usize) -> &Self::Output {
        &self.biases[index]
    }
}

impl IndexMut<usize> for Linear {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.biases[index]
    }
}

impl PartialEq for Linear
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.biases == other.biases
    }
}

impl Linear
{
    fn negate(&self) -> Self {
        Linear::new(self.biases.iter().map(|b| -*b).collect())
    }
}

impl Neg for Linear
{
    type Output = Linear;
    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Neg for &Linear
{
    type Output = Linear;
    fn neg(self) -> Self::Output {
        self.negate()
    }
}

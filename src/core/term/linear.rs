use std::{
    iter::Enumerate,
    ops::{AddAssign, Index, IndexMut},
    slice::{Iter, IterMut},
};

// todo: we need a Linear trait to allow for better interchangeability...
// Currently the expression traits use the structs directly. I don't like this...

#[derive(Clone)]
pub struct Linear<Bias> {
    biases: Vec<Bias>,
}

impl<Bias> Linear<Bias>
where
    // Idx: Into<SizeType>,
    Bias: Default + Clone + Copy + AddAssign,
{
    pub fn default() -> Self {
        Self { biases: Vec::new() }
    }

    pub fn new_from_weighted_variable(var: usize, bias: Bias) -> Self {
        let mut out = Self::default();
        out.biases.insert(var.into(), bias);
        out
    }

    pub fn new_from_variables(lhs: usize, rhs: usize, bias: Bias) -> Self {
        let mut out = Self::default();
        out.biases.insert(lhs, bias);
        out.biases.insert(rhs, bias);
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
        self.biases.resize(new_len, Bias::default());
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
